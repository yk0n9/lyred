use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use chrono::Local;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use portable_atomic::AtomicF32;
use rayon::prelude::*;

use crate::maps::get_map;
use crate::ui::play::Mode;
use crate::{COUNT, LOCAL, PAUSE, PLAYING, POOL, STOP, TIME_SHIFT};

pub static SPEED: AtomicF32 = AtomicF32::new(1.0);
// State:
// 0 -> Stop
// 1 -> Playing
// 2 -> Pause
pub static STATE: AtomicUsize = AtomicUsize::new(STOP);

const DEFAULT_TEMPO_MPQ: f32 = 500000.0;
const DEFAULT_FPS: f32 = 480.0;
const MAP: &'static [i32] = &[
    24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64,
    65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95,
];

#[derive(Debug, Clone)]
pub struct Midi {
    pub name: Arc<Mutex<Option<String>>>,
    pub events: Arc<Mutex<Vec<Event>>>,
    pub fps: Arc<AtomicF32>,
    pub tracks: Arc<Mutex<Vec<Vec<RawEvent>>>>,
    pub track_num: Arc<Mutex<Vec<(bool, usize)>>>,
    pub hit_rate: Arc<AtomicF32>,
}

impl Midi {
    #[inline]
    pub fn new() -> Self {
        Midi {
            name: Arc::new(Mutex::new(None)),
            events: Arc::new(Mutex::new(vec![])),
            fps: Arc::new(Default::default()),
            tracks: Arc::new(Mutex::new(vec![])),
            track_num: Arc::new(Mutex::new(vec![])),
            hit_rate: Arc::new(Default::default()),
        }
    }

    fn play<F: Fn(i32)>(&self, f: F) {
        let events = self.events.lock().unwrap().to_vec();
        let mut start_time = Local::now().timestamp_millis();
        let mut input_time = 0.0;
        let mut i = 0;
        while i < events.len() {
            if TIME_SHIFT.load(Ordering::Relaxed) {
                TIME_SHIFT.store(false, Ordering::Relaxed);
                i = LOCAL.load(Ordering::Relaxed);
                input_time = events[i].delay;
                start_time = Local::now().timestamp_millis();
            } else {
                LOCAL.store(i, Ordering::Relaxed);
            }
            let e = events[i];
            i += 1;

            input_time += e.delay / SPEED.load(Ordering::Relaxed);
            let playback_time = (Local::now().timestamp_millis() - start_time) as f32;
            match (input_time - playback_time) as u64 {
                current @ 1.. => sleep(Duration::from_millis(current)),
                _ => {}
            }
            match STATE.load(Ordering::Relaxed) {
                PLAYING => f(e.press),
                PAUSE => {
                    while STATE.load(Ordering::Relaxed) == PAUSE {}
                    input_time = e.delay;
                    start_time = Local::now().timestamp_millis();
                    i -= 1;
                }
                _ => break,
            }
        }
    }

    pub fn init(self) {
        POOL.spawn(move || {
            if let Some(ref path) = rfd::FileDialog::new()
                .add_filter("MIDI File", &["mid"])
                .pick_file()
            {
                *self.name.lock().unwrap() =
                    Some(path.file_name().unwrap().to_string_lossy().into_owned());

                let file = std::fs::read(path).unwrap();
                let smf = Smf::parse(&file).unwrap();
                let len = smf.tracks.len();
                self.fps.store(
                    match smf.header.timing {
                        Timing::Metrical(fps) => fps.as_int() as f32,
                        _ => DEFAULT_FPS,
                    },
                    Ordering::Relaxed,
                );

                *self.tracks.lock().unwrap() = smf
                    .tracks
                    .into_iter()
                    .map(|track| {
                        let mut tick = 0.0;
                        track
                            .into_iter()
                            .map(|e| {
                                let event = match e.kind {
                                    TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                                        ValidEvent::Tempo(t.as_int() as f32)
                                    }
                                    TrackEventKind::Midi {
                                        message: MidiMessage::NoteOn { key, vel },
                                        ..
                                    } => {
                                        if vel > 0 {
                                            ValidEvent::Note(key.as_int() as i32)
                                        } else {
                                            ValidEvent::Other
                                        }
                                    }
                                    _ => ValidEvent::Other,
                                };
                                tick += e.delta.as_int() as f32;
                                RawEvent { event, tick }
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                self.merge_tracks(&(0..len).collect::<Vec<_>>());
                let enables = vec![true; len].into_iter();
                let range = (0..len).collect::<Vec<_>>().into_iter();
                *self.track_num.lock().unwrap() = enables.zip(range).collect();
            }
            self.hit_rate.store(self.detect(0), Ordering::Relaxed);
        });
    }

    pub fn merge_tracks(&self, indices: &[usize]) {
        let mut current = vec![];
        for (index, events) in self.tracks.lock().unwrap().iter().enumerate() {
            for event in events {
                if indices.contains(&index) {
                    current.push(*event);
                } else {
                    if let ValidEvent::Tempo(_) = event.event {
                        current.push(*event);
                    }
                }
            }
        }
        current.par_sort_by_key(|e| e.tick as usize);

        let mut tick = 0.0;
        let mut tempo = DEFAULT_TEMPO_MPQ;
        let mut time = 0;
        let mut count = Vec::with_capacity(current.len());
        *self.events.lock().unwrap() = current
            .into_iter()
            .filter_map(|event| match event.event {
                ValidEvent::Note(press) => {
                    let delay = Self::tick2millis(
                        event.tick - tick,
                        tempo,
                        self.fps.load(Ordering::Relaxed),
                    );
                    time += delay as usize;
                    count.push(time);
                    tick = event.tick;
                    Some(Event { press, delay })
                }
                ValidEvent::Tempo(t) => {
                    tempo = t;
                    None
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        unsafe {
            COUNT = count;
        }
    }

    pub fn playback(self, offset: i32, mode: Mode) {
        POOL.spawn(move || {
            let send = get_map(mode);
            self.play(|key| send(key + offset));
            STATE.store(STOP, Ordering::Relaxed);
            LOCAL.store(usize::MAX, Ordering::Relaxed);
        });
    }

    pub fn detect(&self, offset: i32) -> f32 {
        let events = self.events.lock().unwrap();
        let all = events.len() as f32;
        let mut count = 0;
        events.iter().for_each(|e| {
            if MAP.contains(&(e.press + offset)) {
                count += 1;
            }
        });
        count as f32 / all
    }

    /// 1. The difference in milliseconds between two events
    /// 2. The time in milliseconds this event was in track
    #[inline]
    fn tick2millis(tick: f32, tempo_mpq: f32, fps: f32) -> f32 {
        tick * tempo_mpq / fps / 1000.0
    }

    /// 1. MPQ-Tempo to BPM-Tempo
    /// 2. BPM-Tempo to MPQ-Tempo
    #[allow(dead_code)]
    #[inline]
    fn convert_tempo(mut tempo: f32) -> f32 {
        if tempo <= 0.0 {
            tempo = 1.0;
        }
        60000000.0 / tempo
    }
}

#[derive(Debug, Copy, Clone)]
enum ValidEvent {
    Note(i32),
    Tempo(f32),
    Other,
}

#[derive(Debug, Copy, Clone)]
pub struct RawEvent {
    event: ValidEvent,
    tick: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub press: i32,
    pub delay: f32,
}
