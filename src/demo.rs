use std::thread::sleep;
use std::time::Duration;
use rs_midi_player::midi::{init, playback};

mod midi;

fn main() {
    let events = init("D:\\song\\だから僕は音楽を辞めた.mid").unwrap();

    println!("Start");
    sleep(Duration::from_millis(500));
    playback(events, 1., true);
    println!("Playback end")
}