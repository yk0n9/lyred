use std::thread::sleep;
use std::time::Duration;
use rs_midi_player::midi::init;
use rs_midi_player::player::Player;


fn main() {
    let events = init("D:\\song\\だから僕は音楽を辞めた.mid").unwrap();

    println!("Start");
    sleep(Duration::from_millis(500));
    let mut play = Player::default();
    play.tuned = true;
    play.playback(events);
    println!("Playback end")
}