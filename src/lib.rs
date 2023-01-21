use enigo::{Enigo, Key, KeyboardControllable};

pub mod midi;
pub mod maps;

pub fn press(click: &mut Enigo, key: Key) {
    click.key_down(key);
    click.key_up(key);
}

pub fn press_precise<'a>(click: &'a mut Enigo, key: &'a str) {
    click.key_sequence(key);
}