pub mod convert;
pub mod font;
pub mod maps;
pub mod midi;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::midi::Midi;
    use crate::ui::Mode;

    #[test]
    fn test() {
        let mid = Midi::new();
        mid.init();

        mid.playback(false, Mode::GenShin);
    }
}
