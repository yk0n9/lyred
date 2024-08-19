use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossbeam::atomic::AtomicCell;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use parking_lot::{Mutex, RwLock};
use rayon::slice::ParallelSliceMut;

use crate::maps::get_map;
use crate::ui::play::Mode;
use crate::{COUNT, LOCAL, POOL, TIME_SHIFT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Playing,
    Stop,
    Pause,
}

pub static STATE: AtomicCell<State> = AtomicCell::new(State::Stop);
pub static SPEED: AtomicCell<f32> = AtomicCell::new(1.0);

const DEFAULT_TEMPO_MPQ: u32 = 500000;
const MAP: &[i32] = &[
    24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64,
    65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95,
];

#[derive(Debug, Clone)]
pub struct Midi {
    pub name: Arc<RwLock<Option<String>>>,
    pub events: Arc<Mutex<Vec<Event>>>,
    pub fps: Arc<AtomicCell<f32>>,
    pub tracks: Arc<Mutex<Vec<Vec<RawEvent>>>>,
    pub track_num: Arc<RwLock<Vec<(bool, usize, String)>>>,
    pub track_keys: Arc<RwLock<Vec<Vec<(usize, i32, i32)>>>>,
    pub hit_rate: Arc<AtomicCell<f32>>,
}

impl Default for Midi {
    fn default() -> Self {
        Self::new()
    }
}

impl Midi {
    #[inline]
    pub fn new() -> Self {
        Midi {
            name: Arc::new(RwLock::new(None)),
            events: Arc::new(Mutex::new(vec![])),
            fps: Arc::new(Default::default()),
            tracks: Arc::new(Mutex::new(vec![])),
            track_num: Arc::new(RwLock::new(vec![])),
            track_keys: Arc::new(RwLock::new(vec![])),
            hit_rate: Arc::new(Default::default()),
        }
    }

    fn play<F: Fn(i32)>(&self, f: F) {
        let events = self.events.lock().to_vec();
        let mut start_time = Instant::now();
        let mut input_time = 0.0;
        let mut i = 0;
        while i < events.len() {
            if TIME_SHIFT.load(Ordering::Relaxed) {
                TIME_SHIFT.store(false, Ordering::Relaxed);
                i = LOCAL.load(Ordering::Relaxed);
                input_time = events[i].delay;
                start_time = Instant::now();
            } else {
                LOCAL.store(i, Ordering::Relaxed);
            }
            let e = events[i];
            i += 1;

            input_time += e.delay / SPEED.load();
            if let current @ 1.. = (input_time - start_time.elapsed().as_micros() as f32) as u64 {
                sleep(Duration::from_micros(current));
            }
            match STATE.load() {
                State::Playing => f(e.press),
                State::Pause => {
                    while STATE.load() == State::Pause {}
                    input_time = e.delay;
                    start_time = Instant::now();
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
                let file = std::fs::read(path).unwrap_or_default();
                let Ok(smf) = Smf::parse(&file) else {
                    return;
                };
                self.name.write().replace(
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                );
                self.fps.store(match smf.header.timing {
                    Timing::Metrical(fps) => fps.as_int() as f32,
                    Timing::Timecode(fps, timing) => (timing & 0xFF) as f32 * fps.as_f32(),
                });
                let track_len = smf.tracks.len();

                let mut track_keys = Vec::with_capacity(track_len);
                let mut track_num = Vec::with_capacity(track_len);
                *self.tracks.lock() = smf
                    .tracks
                    .into_iter()
                    .enumerate()
                    .map(|(index, track)| {
                        let mut tick = 0;
                        let mut track_name = String::from("Untitle");
                        let mut keys = vec![];
                        let events = track
                            .into_iter()
                            .map(|e| {
                                tick += e.delta.as_int();
                                let event = match e.kind {
                                    TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
                                        track_name = String::from_utf8_lossy(name).to_string();
                                        ValidEvent::Other
                                    }
                                    TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                                        ValidEvent::Tempo(t.as_int())
                                    }
                                    TrackEventKind::Meta(MetaMessage::KeySignature(key, _)) => {
                                        keys.push((tick as usize, key as i32, 0));
                                        ValidEvent::Other
                                    }
                                    TrackEventKind::Meta(MetaMessage::EndOfTrack) => {
                                        ValidEvent::Other
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
                                RawEvent { event, tick }
                            })
                            .collect::<Vec<_>>();
                        track_keys.push(keys);
                        track_num.push((true, index, track_name));
                        events
                    })
                    .collect::<Vec<_>>();

                *self.track_keys.write() = track_keys;
                *self.track_num.write() = track_num;
                self.merge_tracks(&(0..track_len).collect::<Vec<_>>(), 0);
            }
        });
    }

    pub fn merge_tracks(&self, indices: &[usize], offset: i32) {
        let mut current = vec![];
        let track_keys = self.track_keys.read();
        println!("{:?}", track_keys.deref());
        for (index, events) in self.tracks.lock().iter_mut().enumerate() {
            let mut last_tick = 0;
            track_keys[index].iter().for_each(|(tick, _, real)| {
                events
                    .iter_mut()
                    .filter(|event| (last_tick..*tick).contains(&(event.tick as usize)))
                    .for_each(|event| {
                        if let ValidEvent::Note(ref mut note) = event.event {
                            *note += *real;
                        }
                    });
                last_tick = *tick;
            });
            for event in events {
                if indices.contains(&index) {
                    current.push(*event);
                } else if let ValidEvent::Tempo(_) = event.event {
                    current.push(*event);
                }
            }
        }
        current.par_sort_by_key(|e| e.tick as usize);

        let mut tick = 0;
        let mut tempo = DEFAULT_TEMPO_MPQ;
        let mut time = 0;
        let mut count = Vec::with_capacity(current.len());
        *self.events.lock() = current
            .into_iter()
            .filter_map(|event| match event.event {
                ValidEvent::Note(press) => {
                    let delay = Self::tick2micros(event.tick - tick, tempo, self.fps.load());
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
            .collect();
        drop(COUNT.take());
        COUNT.store(count);
        self.hit_rate.store(self.detect(offset));
    }

    pub fn playback(self, offset: i32, mode: Mode) {
        POOL.spawn(move || {
            let send = get_map(mode);
            self.play(|key| send(key + offset));
            STATE.store(State::Stop);
            LOCAL.store(!0, Ordering::Relaxed);
        });
    }

    pub fn detect(&self, offset: i32) -> f32 {
        let events = self.events.lock();
        let all = events.len() as f32;
        let mut count = 0;
        events.iter().for_each(|e| {
            if MAP.contains(&(e.press + offset)) {
                count += 1;
            }
        });
        count as f32 / all
    }

    /// 1. The difference in microseconds between two events
    /// 2. The time in microseconds this event was in track
    #[inline]
    fn tick2micros(tick: u32, tempo_mpq: u32, fps: f32) -> f32 {
        (tick * tempo_mpq) as f32 / fps
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

    pub fn current_range(&self) -> Vec<usize> {
        self.track_num
            .read()
            .iter()
            .filter_map(
                |(enable, index, _)| {
                    if *enable {
                        Some(*index)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
enum ValidEvent {
    Note(i32),
    Tempo(u32),
    Other,
}

#[derive(Debug, Copy, Clone)]
pub struct RawEvent {
    event: ValidEvent,
    tick: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub press: i32,
    pub delay: f32,
}
