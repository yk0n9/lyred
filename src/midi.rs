use std::fs::read;
use std::thread::{current, sleep};
use std::time;
use std::time::{Duration, SystemTime};
use anyhow::Result;
use chrono::Local;
use enigo::{Enigo, Key, KeyboardControllable};
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};

struct Event<'a> {
    event: TrackEventKind<'a>,
    tick: u64,
}

pub struct KeyEvent {
    press: Option<char>,
    delay: f64,
}

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
                result.push(KeyEvent { press: c(key.as_int()), delay: time });
            }
        }
    }

    Ok(result)
}

pub fn playback(message: Vec<KeyEvent>, speed: f64) {
    let mut click = Enigo::new();

    let start_time = Local::now().timestamp_millis();
    let mut input_time = 0.;
    for msg in message {
        input_time += msg.delay / speed;

        let playback_time = (Local::now().timestamp_millis() - start_time) as f64;
        let current_time = (input_time - playback_time) as u64;
        if current_time > 0 {
            sleep(Duration::from_millis(current_time));
        }

        match msg.press {
            Some(key) => {
                click.key_down(Key::Layout(key));
                click.key_up(Key::Layout(key));
            }
            _ => {}
        }
    }
}

fn c(key: u8) -> Option<char> {
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