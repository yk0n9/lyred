use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;

use chrono::Local;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use portable_atomic::AtomicF64;
use rayon::prelude::*;

use crate::maps::{gen, vr};
use crate::POOL;
use crate::ui::Mode;

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
    pub tempo: Arc<Mutex<Option<f64>>>,
    pub tracks: Arc<Mutex<Vec<Vec<RawEvent>>>>,
    pub track_num: Arc<AtomicUsize>,
}

impl Midi {
    #[inline]
    pub fn new() -> Self {
        Midi {
            name: Arc::new(Mutex::new(None)),
            events: Arc::new(Mutex::new(vec![])),
            fps: Arc::new(Default::default()),
            tempo: Arc::new(Mutex::new(None)),
            tracks: Arc::new(Mutex::new(vec![])),
            track_num: Arc::new(Default::default()),
        }
    }

    fn play<F: Fn(i32)>(&self, f: F) {
        let mut start_time = Local::now().timestamp_millis();
        let mut input_time = 0.0;
        for e in self.events.lock().unwrap().iter() {
            if PAUSE.load(Ordering::Relaxed) {
                loop {
                    if !PAUSE.load(Ordering::Relaxed) {
                        input_time = e.delay;
                        start_time = Local::now().timestamp_millis();
                        break;
                    }
                }
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
                self.track_num.store(len, Ordering::Relaxed);
                *self.tempo.lock().unwrap() = None;
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
                            .filter_map(|e| {
                                let event = match e.kind {
                                    TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                                        let mut tempo = self.tempo.lock().unwrap();
                                        if tempo.is_none() {
                                            *tempo = Some(t.as_int() as f64);
                                        }
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
                                Some(RawEvent {
                                    event,
                                    tick,
                                })
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                self.merge_tracks(&(0..len).collect::<Vec<_>>());
            }
        });
    }

    pub fn merge_tracks(&self, indices: &[usize]) {
        let mut current = vec![];
        for i in indices {
            for event in self.tracks.lock().unwrap()[*i].iter() {
                current.push(*event);
            }
        }
        current.par_sort_by_key(|e| e.tick as usize);

        let mut tick = 0.0;
        let mut tempo = if let Some(tempo) = *self.tempo.lock().unwrap() {
            tempo
        } else {
            500000.0
        };
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

    pub fn playback(self, tuned: bool, mode: Mode) {
        PLAYING.store(true, Ordering::Relaxed);
        POOL.spawn(move || {
            let mut shift = 0;
            if tuned {
                shift = tune(self.events.clone());
            }
            let send = match mode {
                Mode::GenShin => gen,
                Mode::VRChat => vr,
            };
            self.play(|key| {
                send(key + shift);
            });
            PLAYING.store(false, Ordering::Relaxed);
            IS_PLAY.store(false, Ordering::Relaxed);
        });
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

fn tune(events: Arc<Mutex<Vec<Event>>>) -> i32 {
    let len = events.lock().unwrap().len() as f32;
    let mut up_hit = vec![];
    let mut down_hit = vec![];
    let mut up_max = 0.0;
    let mut down_max = 0.0;
    let mut up_shift = 0;
    let mut down_shift = 0;

    rayon::join(
        || {
            tune_offset(events.clone(), len, &mut up_hit, 0, true);
            for (i, x) in up_hit.into_iter().enumerate() {
                if x > up_max {
                    up_max = x;
                    up_shift = i as i32;
                }
            }
        },
        || {
            tune_offset(events.clone(), len, &mut down_hit, 0, false);
            for (i, x) in down_hit.into_iter().enumerate() {
                if x > down_max {
                    down_max = x;
                    down_shift = i as i32;
                }
            }
        },
    );

    if up_shift > down_shift {
        return up_shift;
    }
    -down_shift
}

fn tune_offset(
    events: Arc<Mutex<Vec<Event>>>,
    len: f32,
    hit_vec: &mut Vec<f32>,
    offset: i32,
    direction: bool,
) {
    let mut hit_count = 0.0;
    for msg in events.lock().unwrap().iter() {
        let key = msg.press + offset;
        if MAP.contains(&key) {
            hit_count += 1.0;
        }
    }
    let hit = hit_count / len;
    hit_vec.push(hit);
    match direction {
        true => {
            if offset > 21 {
                return;
            }
            tune_offset(events, len, hit_vec, offset + 1, true);
        }
        false => {
            if offset < -21 {
                return;
            }
            tune_offset(events, len, hit_vec, offset - 1, false);
        }
    }
}
