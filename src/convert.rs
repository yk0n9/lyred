use std::{fs, thread};
use crate::Data;
use crate::midi::KeyEvent;

pub fn convert_from_midi(data: Data<Vec<KeyEvent>>) {
    let _ = thread::spawn(move || {
        let mut res = String::new();
        for e in data.lock().unwrap().iter() {
            match e.press {
                24 => res.push('z'),
                26 => res.push('x'),
                28 => res.push('c'),
                29 => res.push('v'),
                31 => res.push('b'),
                33 => res.push('n'),
                35 => res.push('m'),
                36 => res.push('z'),
                38 => res.push('x'),
                40 => res.push('c'),
                41 => res.push('v'),
                43 => res.push('b'),
                45 => res.push('n'),
                47 => res.push('m'),
                48 => res.push('z'),
                50 => res.push('x'),
                52 => res.push('c'),
                53 => res.push('v'),
                55 => res.push('b'),
                57 => res.push('n'),
                59 => res.push('m'),
                60 => res.push('a'),
                62 => res.push('s'),
                64 => res.push('d'),
                65 => res.push('f'),
                67 => res.push('g'),
                69 => res.push('h'),
                71 => res.push('j'),
                72 => res.push('q'),
                74 => res.push('w'),
                76 => res.push('e'),
                77 => res.push('r'),
                79 => res.push('t'),
                81 => res.push('y'),
                83 => res.push('u'),
                84 => res.push('q'),
                86 => res.push('w'),
                88 => res.push('e'),
                89 => res.push('r'),
                91 => res.push('t'),
                93 => res.push('y'),
                95 => res.push('u'),
                _ => {}
            }
            match e.delay {
                e if e > 50.0 && e <= 700.0 => res.push_str(" - "),
                e if e > 700.0 && e <= 2000.0 => res.push_str(" --- "),
                e if e > 2000.0 => res.push_str("\n\n"),
                _ => {}
            }
        }
        let mut phone = String::new();
        for c in res.chars() {
            match c {
                'q' => phone.push_str("+1"),
                'w' => phone.push_str("+2"),
                'e' => phone.push_str("+3"),
                'r' => phone.push_str("+4"),
                't' => phone.push_str("+5"),
                'y' => phone.push_str("+6"),
                'u' => phone.push_str("+7"),
                'a' => phone.push_str("1"),
                's' => phone.push_str("2"),
                'd' => phone.push_str("3"),
                'f' => phone.push_str("4"),
                'g' => phone.push_str("5"),
                'h' => phone.push_str("6"),
                'j' => phone.push_str("7"),
                'z' => phone.push_str("-1"),
                'x' => phone.push_str("-2"),
                'c' => phone.push_str("-3"),
                'v' => phone.push_str("-4"),
                'b' => phone.push_str("-5"),
                'n' => phone.push_str("-6"),
                'm' => phone.push_str("-7"),
                _ => phone.push(c),
            }
        }
        fs::write("keyboard.txt", &res.to_uppercase()).unwrap();
        fs::write("phone.txt", &phone).unwrap();
    });
}