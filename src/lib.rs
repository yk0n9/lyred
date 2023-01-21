use enigo::{Enigo, Key, KeyboardControllable};

pub mod midi;
pub mod maps;

pub fn press(click: &mut Enigo, key: Key) {
    click.key_down(key);
    click.key_up(key);
}
