use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use std::fs::read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use enigo::Enigo;
use crate::maps::{GEN_SHIN, VR_CHAT};

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    GenShin,
    VRChat,
}

struct Event<'a> {
    event: TrackEventKind<'a>,
    tick: f64,
}

#[derive(Clone)]
pub struct KeyEvent {
    pub press: u8,
    pub delay: f64,
}

const MAP: [i32; 42] = [
    24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64,
    65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95,
];

pub fn init(opened_file: Arc<Mutex<Option<PathBuf>>>, key_events: Arc<Mutex<Vec<KeyEvent>>>) {
    let _ = thread::spawn(move || {
        let mut path = PathBuf::new();
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("MIDI File", &["mid"])
            .pick_file()
        {
            path = file.clone();
            *opened_file.lock().unwrap() = Some(file);
        }
        let file = read(path).unwrap();
        let midi = Smf::parse(&file).expect("Not a Midi File");
        let resolution = match midi.header.timing {
            Timing::Metrical(resolution) => resolution.as_int() as f64,
            _ => unimplemented!()
        };
        let mut events = vec![];
        let mut result = vec![];

        midi.tracks.into_iter().for_each(|track| {
            let mut tick = 0.0;

            for event in track {
                tick += event.delta.as_int() as f64;

                events.push(Event {
                    event: event.kind,
                    tick,
                });
            }
        });

        events.sort_by_key(|e| e.tick as u64);

        let mut tick = 0.0;
        let mut tempo = 500000.0;
        events.into_iter().for_each(|event| {
            let time: f64;

            match event.event {
                TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                    tempo = t.as_int() as f64;
                }
                TrackEventKind::Midi {
                    channel: _,
                    message: MidiMessage::NoteOn { key, vel },
                } => {
                    if vel > 0 {
                        time = (event.tick - tick) * (tempo / 1000.0 / resolution);
                        tick = event.tick;
                        result.push(KeyEvent {
                            press: key.as_int(),
                            delay: time,
                        });
                    }
                }
                _ => {}
            }
        });
        *key_events.lock().unwrap() = result;
    });
}

pub fn tune(message: Arc<Mutex<Vec<KeyEvent>>>) -> i32 {
    let len = message.lock().unwrap().len() as f32;
    let mut up_hit = vec![];
    let mut down_hit = vec![];
    let mut up_max = 0.0;
    let mut down_max = 0.0;
    let mut up_shift = 0;
    let mut down_shift = 0;

    rayon::join(
        || tune_offset(message.clone(), len, &mut up_hit, 0, true),
        || tune_offset(message.clone(), len, &mut down_hit, 0, false),
    );

    for (i, x) in up_hit.into_iter().enumerate() {
        if x > up_max {
            up_max = x;
            up_shift = i as i32;
        }
    }
    for (i, x) in down_hit.into_iter().enumerate() {
        if x > down_max {
            down_max = x;
            down_shift = i as i32;
        }
    }

    if up_shift > down_shift {
        return up_shift;
    }
    -down_shift
}

fn tune_offset(message: Arc<Mutex<Vec<KeyEvent>>>, len: f32, hit_vec: &mut Vec<f32>, offset: i32, direction: bool) {
    let mut hit_count = 0.0;
    for msg in message.lock().unwrap().iter() {
        let key = msg.press as i32 + offset;
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
            tune_offset(message, len, hit_vec, offset + 1, true);
        }
        _ => {
            if offset < -21 {
                return;
            }
            tune_offset(message, len, hit_vec, offset - 1, false);
        }
    }
}

pub fn playback(
    message: Arc<Mutex<Vec<KeyEvent>>>,
    tuned: bool,
    speed: Arc<Mutex<f64>>,
    is_play: Arc<Mutex<bool>>,
    pause: Arc<Mutex<bool>>,
    mode: Mode,
) {
    let _ = thread::spawn(move || {
        let mut click = Enigo::new();
        let mut shift = 0;
        let send = match mode {
            Mode::GenShin => GEN_SHIN,
            Mode::VRChat => VR_CHAT
        };

        if tuned {
            shift = tune(message.clone());
        }

        let mut start_time = Local::now().timestamp_millis();
        let mut input_time = 0.0;
        for msg in message.lock().unwrap().iter() {
            if *pause.lock().unwrap() {
                loop {
                    if !*pause.lock().unwrap() {
                        input_time = msg.delay;
                        start_time = Local::now().timestamp_millis();
                        break;
                    }
                }
            }

            if !*is_play.lock().unwrap() {
                break;
            }

            input_time += msg.delay / *speed.lock().unwrap();

            let playback_time = (Local::now().timestamp_millis() - start_time) as f64;
            let current_time = (input_time - playback_time) as u64;
            if current_time > 0 {
                sleep(Duration::from_millis(current_time));
            }

            send(&mut click, (msg.press as i32 + shift) as u8);
        }
        *is_play.lock().unwrap() = false;
    });
}


