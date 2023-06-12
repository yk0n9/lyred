pub mod convert;
pub mod font;
pub mod maps;
pub mod midi;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::midi::{init, Midi, playback};

    #[test]
    fn test() {
        let mid = Midi::new();
        init(mid.clone());

        playback(mid.clone(), false, 0);
    }
}