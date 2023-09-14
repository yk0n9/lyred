use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

use chrono::Local;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use portable_atomic::AtomicF64;
use rayon::prelude::*;

use crate::maps::{gen, vr};
use crate::POOL;
use crate::ui::play::Mode;

pub static SPEED: AtomicF64 = AtomicF64::new(1.0);
pub static IS_PLAY: AtomicBool = AtomicBool::new(false);
pub static PLAYING: AtomicBool = AtomicBool::new(false);
pub static PAUSE: AtomicBool = AtomicBool::new(false);

const MAP: &'static [i32] = &[
    24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64,
    65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95,
];

#[derive(Debug, Clone)]
pub struct Midi {
    pub name: Arc<Mutex<Option<String>>>,
    pub events: Arc<Mutex<Vec<Event>>>,
    pub fps: Arc<AtomicF64>,
    pub tracks: Arc<Mutex<Vec<Vec<RawEvent>>>>,
    pub track_num: Arc<Mutex<Vec<(bool, usize)>>>,
    pub hit_rate: Arc<AtomicF64>,
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
        for e in events.into_iter() {
            if PAUSE.load(Ordering::Relaxed) {
                while PAUSE.load(Ordering::Relaxed) {}
                input_time = e.delay;
                start_time = Local::now().timestamp_millis();
            }
            input_time += e.delay / SPEED.load(Ordering::Relaxed);
            let playback_time = (Local::now().timestamp_millis() - start_time) as f64;
            let current_time = (input_time - playback_time) as u64;
            if current_time > 0 {
                sleep(Duration::from_millis(current_time));
            }
            match IS_PLAY.load(Ordering::Relaxed) {
                true => f(e.press),
                false => break,
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
                self.fps.store(match smf.header.timing {
                    Timing::Metrical(fps) => fps.as_int() as f64,
                    _ => 480.0,
                }, Ordering::Relaxed);

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
                                        ValidEvent::Tempo(t.as_int() as f64)
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
                                tick += e.delta.as_int() as f64;
                                RawEvent {
                                    event,
                                    tick,
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                self.merge_tracks(&(0..len).collect::<Vec<_>>());
                let enables = vec![true; len].into_iter();
                let range = (0..len).collect::<Vec<_>>().into_iter();
                *self.track_num.lock().unwrap() = enables.zip(range).collect();
            }
            self.hit_rate.store(self.detection(0), Ordering::Relaxed);
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
        let mut tempo = 500000.0;
        *self.events.lock().unwrap() = current
            .into_iter()
            .filter_map(|event| match event.event {
                ValidEvent::Note(press) => {
                    let delay = (event.tick - tick) * (tempo / 1000.0 / self.fps.load(Ordering::Relaxed));
                    tick = event.tick;
                    Some(Event {
                        press,
                        delay,
                    })
                }
                ValidEvent::Tempo(t) => {
                    tempo = t;
                    None
                }
                _ => None,
            })
            .collect::<Vec<_>>();
    }

    pub fn playback(self, offset: i32, mode: Mode) {
        PLAYING.store(true, Ordering::Relaxed);
        POOL.spawn(move || {
            let send = match mode {
                Mode::GenShin => gen,
                Mode::VRChat => vr,
            };
            self.play(|key| {
                send(key + offset);
            });
            PLAYING.store(false, Ordering::Relaxed);
            IS_PLAY.store(false, Ordering::Relaxed);
        });
    }

    pub fn detection(&self, offset: i32) -> f64 {
        let events = self.events.lock().unwrap();
        let all = events.len() as f64;
        let mut count = 0;
        events.iter().for_each(|e| {
            if MAP.contains(&(e.press + offset)) {
                count += 1;
            }
        });
        count as f64 / all
    }
}

#[derive(Debug, Copy, Clone)]
enum ValidEvent {
    Note(i32),
    Tempo(f64),
    Other,
}

#[derive(Debug, Copy, Clone)]
pub struct RawEvent {
    event: ValidEvent,
    tick: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub press: i32,
    pub delay: f64,
}
