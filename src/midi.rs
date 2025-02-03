use std::path::Path;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossbeam::atomic::AtomicCell;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use parking_lot::RwLock;
use rand::Rng;
use rayon::slice::ParallelSliceMut;

use crate::maps::get_map;
use crate::ui::play::{Mode, PlayMode};
use crate::{COUNT, LOCAL, POOL, TIME_SHIFT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Playing,
    Stop,
    Pause,
}

pub static PLAYING: AtomicCell<bool> = AtomicCell::new(false);
pub static STATE: AtomicCell<State> = AtomicCell::new(State::Stop);
pub static SPEED: AtomicCell<f32> = AtomicCell::new(1.0);
pub static CURRENT_MIDI: AtomicCell<usize> = AtomicCell::new(0);

pub fn is_playing() -> bool {
    !matches!(STATE.load(), State::Stop)
}

#[derive(Debug, Clone)]
pub struct Midi {
    pub name: Arc<RwLock<Option<String>>>,
    pub events: Arc<RwLock<Vec<Event>>>,
    pub fps: Arc<AtomicCell<f32>>,
    pub tracks: Arc<RwLock<Vec<Vec<RawEvent>>>>,
    pub track_num: Arc<RwLock<Vec<(bool, usize, String)>>>,
    pub track_keys: Arc<RwLock<Vec<Vec<TrackKey>>>>,
    pub hit_rate: Arc<AtomicCell<f32>>,
    pub midis: Arc<RwLock<Vec<String>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct TrackKey {
    pub tick: u32,
    pub key: i32,
    pub backup: i32,
    pub real: i32,
}

impl Default for Midi {
    fn default() -> Self {
        Self::new()
    }
}

impl Midi {
    #[inline]
    pub fn new() -> Self {
        Midi {
            name: Arc::new(RwLock::new(None)),
            events: Arc::new(RwLock::new(vec![])),
            fps: Arc::new(Default::default()),
            tracks: Arc::new(RwLock::new(vec![])),
            track_num: Arc::new(RwLock::new(vec![])),
            track_keys: Arc::new(RwLock::new(vec![])),
            hit_rate: Arc::new(Default::default()),
            midis: Arc::new(RwLock::new(vec![])),
        }
    }

    fn play<F: Fn(i32)>(&self, offset: i32, f: F) {
        let events = self.events.read();
        let mut start_time = Instant::now();
        let mut input_time = 0.0;
        let mut i = 0;
        while i < events.len() {
            if TIME_SHIFT.load() {
                TIME_SHIFT.store(false);
                i = LOCAL.load();
                input_time = events[i].delay;
                start_time = Instant::now();
            } else {
                LOCAL.store(i);
            }
            let e = events[i];
            i += 1;

            input_time += e.delay / SPEED.load();
            if let current @ 1.. = (input_time - start_time.elapsed().as_micros() as f32) as u64 {
                sleep(Duration::from_micros(current));
            }
            match STATE.load() {
                State::Playing => f(e.press + offset),
                State::Pause => {
                    while STATE.load() == State::Pause {}
                    input_time = e.delay;
                    start_time = Instant::now();
                    i -= 1;
                }
                State::Stop => break,
            }
        }
    }

    pub fn init(self) {
        POOL.spawn(move || {
            if let Some(ref path) = rfd::FileDialog::new()
                .add_filter("MIDI File", &["mid"])
                .pick_file()
            {
                self.read_midi(path);
            }
        });
    }

    pub fn read_midi(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let file = std::fs::read(path).unwrap_or_default();
        let Ok(smf) = Smf::parse(&file) else {
            return;
        };
        self.name.write().replace(
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
        );
        self.fps.store(match smf.header.timing {
            Timing::Metrical(fps) => fps.as_int() as f32,
            Timing::Timecode(fps, timing) => timing as f32 * fps.as_f32(),
        });
        let track_len = smf.tracks.len();

        let mut track_keys = Vec::with_capacity(track_len);
        let mut track_num = Vec::with_capacity(track_len);
        *self.tracks.write() = smf
            .tracks
            .into_iter()
            .enumerate()
            .map(|(index, track)| {
                let mut tick = 0;
                let mut track_name = String::from("Untitle");
                let mut keys = vec![];
                let events = track
                    .into_iter()
                    .map(|e| {
                        tick += e.delta.as_int();
                        let event = match e.kind {
                            TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
                                track_name = String::from_utf8_lossy(name).to_string();
                                ValidEvent::Other
                            }
                            TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                                ValidEvent::Tempo(t.as_int())
                            }
                            TrackEventKind::Meta(MetaMessage::KeySignature(key, _)) => {
                                keys.push(TrackKey {
                                    tick,
                                    key: key as i32,
                                    backup: key as i32,
                                    real: 0,
                                });
                                ValidEvent::Other
                            }
                            TrackEventKind::Midi {
                                message: MidiMessage::NoteOn { key, vel },
                                ..
                            } => {
                                if vel > 0 {
                                    ValidEvent::Note(key.as_int() as i32)
                                } else {
                                    ValidEvent::Other
                                }
                            }
                            _ => ValidEvent::Other,
                        };
                        RawEvent { event, tick }
                    })
                    .collect::<Vec<_>>();
                track_keys.push(keys);
                track_num.push((true, index, track_name));
                events
            })
            .collect::<Vec<_>>();

        *self.track_keys.write() = track_keys;
        *self.track_num.write() = track_num;
        self.merge_tracks(&(0..track_len).collect::<Vec<_>>(), 0);
    }

    pub fn merge_tracks(&self, indices: &[usize], offset: i32) {
        const DEFAULT_TEMPO_MPQ: u32 = 500000;

        let mut current = vec![];
        let mut tracks = self.tracks.read().to_vec();
        let track_keys = self.track_keys.read();
        for (index, events) in tracks.iter_mut().enumerate() {
            let mut keys = track_keys[index].iter().peekable();
            let mut next = keys.next();
            events.iter_mut().for_each(|event| {
                let mut cond = true;
                if let Some(next) = next {
                    cond = if let Some(peek) = keys.peek() {
                        (next.tick..peek.tick).contains(&event.tick)
                    } else {
                        (next.tick..).contains(&event.tick)
                    };
                    if let ValidEvent::Note(ref mut note) = event.event {
                        if cond {
                            *note += next.real;
                        }
                    }
                }
                if !cond {
                    next = keys.next();
                }

                if indices.contains(&index) || event.event.is_tempo() {
                    current.push(*event);
                }
            });
        }
        current.par_sort_by_key(|e| e.tick);

        let mut tick = 0;
        let mut tempo = DEFAULT_TEMPO_MPQ;
        let mut time = 0;
        let mut count = Vec::with_capacity(current.len());
        *self.events.write() = current
            .into_iter()
            .filter_map(|event| match event.event {
                ValidEvent::Note(press) => {
                    let delay = Self::tick2micros(event.tick - tick, tempo, self.fps.load());
                    time += delay as usize;
                    count.push(time);
                    tick = event.tick;
                    Some(Event { press, delay })
                }
                ValidEvent::Tempo(t) => {
                    tempo = t;
                    None
                }
                _ => None,
            })
            .collect();
        *COUNT.write() = count;
        self.hit_rate.store(self.detect(offset));
    }

    pub fn playback(&self, offset: i32, mode: Mode) {
        let send = get_map(mode);
        PLAYING.store(true);
        self.play(offset, send);
        PLAYING.store(false);
        LOCAL.store(0);
    }

    pub fn playback_one(self, offset: i32, mode: Mode, once: bool) {
        POOL.spawn(move || {
            loop {
                self.playback(offset, mode);
                if STATE.load() == State::Stop || !once {
                    break;
                }
            }
            STATE.store(State::Stop);
        });
    }

    pub fn playback_list(
        self,
        mut index: usize,
        dir_path: impl AsRef<Path>,
        mode: Mode,
        random: bool,
    ) {
        let path = dir_path.as_ref().to_path_buf();
        let max = self.midis.read().len();
        POOL.spawn(move || loop {
            if index < max {
                let midis = self.midis.read();
                let file = midis[index].as_str();
                CURRENT_MIDI.store(index);
                self.read_midi(path.join(file));
                self.playback(0, mode);
                if let State::Stop = STATE.load() {
                    break;
                }
                if random {
                    let mut rng = rand::rng();
                    index = rng.random_range(0..max);
                } else {
                    index += 1;
                }
            } else {
                index = 0;
            }
        });
    }

    pub fn playback_by(self, path: impl AsRef<Path>, offset: i32, play_mode: PlayMode, mode: Mode) {
        match play_mode {
            PlayMode::Once | PlayMode::OneLoop => {
                STATE.store(State::Playing);
                self.playback_one(offset, mode, play_mode.eq(&PlayMode::OneLoop));
            }
            PlayMode::Loop | PlayMode::Random => {
                if !self.midis.read().is_empty() {
                    STATE.store(State::Playing);
                    self.playback_list(
                        CURRENT_MIDI.load(),
                        path,
                        mode,
                        play_mode.eq(&PlayMode::Random),
                    );
                }
            }
        }
    }

    pub fn detect(&self, offset: i32) -> f32 {
        const MAP: &[i32] = &[
            24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48, 50, 52, 53, 55, 57, 59, 60,
            62, 64, 65, 67, 69, 71, 72, 74, 76, 77, 79, 81, 83, 84, 86, 88, 89, 91, 93, 95,
        ];

        let events = self.events.read();
        let all = events.len() as f32;
        let mut count = 0;
        events.iter().for_each(|e| {
            if MAP.contains(&(e.press + offset)) {
                count += 1;
            }
        });
        count as f32 / all
    }

    /// 1. The difference in microseconds between two events
    /// 2. The time in microseconds this event was in track
    #[inline]
    fn tick2micros(tick: u32, tempo_mpq: u32, fps: f32) -> f32 {
        tick as f32 * tempo_mpq as f32 / fps
    }

    /// 1. MPQ-Tempo to BPM-Tempo
    /// 2. BPM-Tempo to MPQ-Tempo
    #[allow(dead_code)]
    #[inline]
    fn convert_tempo(mut tempo: f32) -> f32 {
        if tempo <= 0.0 {
            tempo = 1.0;
        }
        60000000.0 / tempo
    }

    pub fn current_range(&self) -> Vec<usize> {
        self.track_num
            .read()
            .iter()
            .filter_map(
                |(enable, index, _)| {
                    if *enable {
                        Some(*index)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    pub fn get_midis_path(&self, path: impl AsRef<Path>) {
        let Ok(entry) = path.as_ref().read_dir() else {
            return;
        };
        let midis = entry
            .into_iter()
            .filter_map(|entry| {
                let Ok(entry) = entry else {
                    return None;
                };
                if entry.path().extension().unwrap_or_default().eq("mid") {
                    Some(entry.file_name().to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        CURRENT_MIDI.store(0);
        *self.midis.write() = midis;
    }

    pub fn switch_midi(&self, index: usize, path: impl AsRef<Path>) {
        CURRENT_MIDI.store(index);
        STATE.store(State::Stop);
        self.read_midi(path);
    }
}

#[derive(Debug, Copy, Clone)]
enum ValidEvent {
    Note(i32),
    Tempo(u32),
    Other,
}

impl ValidEvent {
    fn is_tempo(&self) -> bool {
        matches!(self, ValidEvent::Tempo(_))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RawEvent {
    event: ValidEvent,
    tick: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub press: i32,
    pub delay: f32,
}
