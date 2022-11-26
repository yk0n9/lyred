use std::fs::read;
use std::thread::sleep;
use std::time::Duration;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use anyhow::Result;
use chrono::Local;
use enigo::{Enigo, Key, KeyboardControllable};

struct Event<'a> {
    event: TrackEventKind<'a>,
    tick: u64,
}

#[derive(Clone)]
pub struct KeyEvent {
    pub press: u8,
    pub delay: f64,
}

const MAP: [i32; 42] = [24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64, 65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95];

pub fn init(path: &str) -> Result<Vec<KeyEvent>> {
    let file = read(path).unwrap();
    let midi = Smf::parse(&file).unwrap();
    let resolution = match midi.header.timing {
        Timing::Metrical(resolution) => { resolution.as_int() as f64 }
        _ => { unimplemented!() }
    };
    let mut events = vec![];
    let mut result = vec![];

    for track in midi.tracks {
        let mut tick = 0.;

        for event in track {
            tick += event.delta.as_int() as f64;
            events.push(Event {
                event: event.kind,
                tick: tick as u64,
            })
        }
    }

    events.sort_by_key(|e| e.tick);

    let mut tick = 0.;
    let mut tempo = 500000.;
    for event in events {
        let mut time: f64;

        if let TrackEventKind::Meta(MetaMessage::Tempo(t)) = event.event {
            tempo = t.as_int() as f64;
        }

        if let TrackEventKind::Midi { channel: _, message: MidiMessage::NoteOn { key, vel } } = event.event {
            if vel > 0 {
                time = (event.tick as f64 - tick) * (tempo / 1000. / resolution);
                tick = event.tick as f64;
                result.push(KeyEvent { press: key.as_int(), delay: time });
            }
        }
    }

    Ok(result)
}

// pub fn playback(message: Vec<KeyEvent>, speed: f64, tuned: bool) {
//     let mut click = Enigo::new();
//
//     let mut shift = 0;
//
//     if tuned {
//         shift = tune(message.clone());
//     }
//
//     let start_time = Local::now().timestamp_millis();
//     let mut input_time = 0.;
//     for msg in message.into_iter() {
//         input_time += msg.delay / speed;
//
//         let playback_time = (Local::now().timestamp_millis() - start_time) as f64;
//         let current_time = (input_time - playback_time) as u64;
//         if current_time > 0 {
//             sleep(Duration::from_millis(current_time));
//         }
//
//         match c((msg.press as i32 + shift) as u8) {
//             Some(key) => {
//                 click.key_down(Key::Layout(key));
//                 click.key_up(Key::Layout(key));
//             }
//             _ => {}
//         }
//     }
// }

pub fn tune(message: Vec<KeyEvent>) -> i32 {
    let mut up_hit = vec![];
    let mut down_hit = vec![];
    tune_up(message.clone(), &mut up_hit, 0);
    tune_down(message.clone(), &mut down_hit, 0);

    let mut up_max = 0.;
    let mut down_max = 0.;
    let mut up_shift = 0;
    let mut down_shift = 0;
    for (i, x) in up_hit.iter().enumerate() {
        if *x > up_max {
            up_max = *x;
            up_shift = i as i32;
        }
    }
    for (i, x) in down_hit.iter().enumerate() {
        if *x > down_max {
            down_max = *x;
            down_shift = i as i32;
        }
    }

    if up_shift > down_shift {
        println!("Hit: {}", up_max);
        return up_shift;
    }
    println!("Hit: {}", down_max);
    -down_shift
}

fn tune_up(message: Vec<KeyEvent>, hit_vec: &mut Vec<f32>, offset: i32) {
    let mut hit_count = 0.;
    let len = message.len() as f32;
    for msg in &message {
        let key = msg.press as i32 + offset;
        if MAP.contains(&key) {
            hit_count += 1.;
        }
    }
    let hit = hit_count / len;
    hit_vec.push(hit);

    if offset > 21 {
        return;
    }
    tune_up(message, hit_vec, offset + 1);
}

fn tune_down(message: Vec<KeyEvent>, hit_vec: &mut Vec<f32>, offset: i32) {
    let mut hit_count = 0.;
    let len = message.len() as f32;
    for msg in &message {
        let key = msg.press as i32 + offset;
        if MAP.contains(&key) {
            hit_count += 1.;
        }
    }
    let hit = hit_count / len;
    hit_vec.push(hit);

    if offset < -21 {
        return;
    }
    tune_down(message, hit_vec, offset - 1);
}

pub fn c(key: u8) -> Option<char> {
    Some(
        match key {
            24 => 'z',
            26 => 'x',
            28 => 'c',
            29 => 'v',
            31 => 'b',
            33 => 'n',
            35 => 'm',
            36 => 'z',
            38 => 'x',
            40 => 'c',
            41 => 'v',
            43 => 'b',
            45 => 'n',
            47 => 'm',
            48 => 'z',
            50 => 'x',
            52 => 'c',
            53 => 'v',
            55 => 'b',
            57 => 'n',
            59 => 'm',
            60 => 'a',
            62 => 's',
            64 => 'd',
            65 => 'f',
            67 => 'g',
            69 => 'h',
            71 => 'j',
            72 => 'q',
            74 => 'w',
            76 => 'e',
            77 => 'r',
            79 => 't',
            81 => 'y',
            83 => 'u',
            84 => 'q',
            86 => 'w',
            88 => 'e',
            89 => 'r',
            91 => 't',
            93 => 'y',
            95 => 'u',
            _ => return None
        })
}