use std::mem;

use strum::{EnumIter, IntoEnumIterator};
use winapi::um::winuser::{
    GetAsyncKeyState, INPUT_u, MapVirtualKeyA, SendInput, INPUT, INPUT_KEYBOARD, INPUT_MOUSE,
    KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MAPVK_VK_TO_VSC, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEINPUT,
};

use crate::{ui::play::Mode, util::VKey};

pub static mut MAP: [VKey; 21] = [
    VKey::Q,
    VKey::W,
    VKey::E,
    VKey::R,
    VKey::T,
    VKey::Y,
    VKey::U,
    VKey::A,
    VKey::S,
    VKey::D,
    VKey::F,
    VKey::G,
    VKey::H,
    VKey::J,
    VKey::Z,
    VKey::X,
    VKey::C,
    VKey::V,
    VKey::B,
    VKey::N,
    VKey::M,
];

#[inline]
pub fn is_pressed(vk: VKey) -> bool {
    unsafe { GetAsyncKeyState(vk as _) >> 15 != 0 }
}

#[inline]
pub fn get_map(mode: Mode) -> impl Fn(i32, bool) {
    match mode {
        Mode::GenShin => gen_shin,
        Mode::VRChat => vr_chat,
        Mode::DeltaForce => delta_force,
    }
}

#[inline]
fn handle_key(vk: VKey, is_on: bool) {
    if is_on {
        press(vk);
    } else {
        release(vk);
    }
}

#[inline]
pub fn gen_shin(val: i32, is_on: bool) {
    if !is_on {
        return;
    }

    unsafe {
        match val {
            24 => click(MAP[14]),
            26 => click(MAP[15]),
            28 => click(MAP[16]),
            29 => click(MAP[17]),
            31 => click(MAP[18]),
            33 => click(MAP[19]),
            35 => click(MAP[20]),

            36 => click(MAP[14]),
            38 => click(MAP[15]),
            40 => click(MAP[16]),
            41 => click(MAP[17]),
            43 => click(MAP[18]),
            45 => click(MAP[19]),
            47 => click(MAP[20]),

            48 => click(MAP[14]),
            50 => click(MAP[15]),
            52 => click(MAP[16]),
            53 => click(MAP[17]),
            55 => click(MAP[18]),
            57 => click(MAP[19]),
            59 => click(MAP[20]),

            60 => click(MAP[7]),
            62 => click(MAP[8]),
            64 => click(MAP[9]),
            65 => click(MAP[10]),
            67 => click(MAP[11]),
            69 => click(MAP[12]),
            71 => click(MAP[13]),

            72 => click(MAP[0]),
            74 => click(MAP[1]),
            76 => click(MAP[2]),
            77 => click(MAP[3]),
            79 => click(MAP[4]),
            81 => click(MAP[5]),
            83 => click(MAP[6]),

            84 => click(MAP[0]),
            86 => click(MAP[1]),
            88 => click(MAP[2]),
            89 => click(MAP[3]),
            91 => click(MAP[4]),
            93 => click(MAP[5]),
            95 => click(MAP[6]),
            _ => {}
        };
    }
}

#[inline]
pub fn vr_chat(val: i32, is_on: bool) {
    if !is_on {
        return;
    }

    match val {
        36 => click(VKey::Z),
        37 => click(VKey::Comma),
        38 => click(VKey::X),
        39 => click(VKey::Period),
        40 => click(VKey::C),
        41 => click(VKey::V),
        42 => click(VKey::Slash),
        43 => click(VKey::B),
        44 => click(VKey::Np0),
        45 => click(VKey::N),
        46 => click(VKey::NpDecimal),
        47 => click(VKey::M),

        48 => click(VKey::A),
        49 => click(VKey::K),
        50 => click(VKey::S),
        51 => click(VKey::L),
        52 => click(VKey::D),
        53 => click(VKey::F),
        54 => click(VKey::Semicolon),
        55 => click(VKey::G),
        56 => click(VKey::Np2),
        57 => click(VKey::H),
        58 => click(VKey::Np3),
        59 => click(VKey::J),

        60 => click(VKey::Q),
        61 => click(VKey::I),
        62 => click(VKey::W),
        63 => click(VKey::O),
        64 => click(VKey::E),
        65 => click(VKey::R),
        66 => click(VKey::P),
        67 => click(VKey::T),
        68 => click(VKey::Np5),
        69 => click(VKey::Y),
        70 => click(VKey::Np6),
        71 => click(VKey::U),

        72 => click(VKey::Num1),
        73 => click(VKey::Num8),
        74 => click(VKey::Num2),
        75 => click(VKey::Num9),
        76 => click(VKey::Num3),
        77 => click(VKey::Num4),
        78 => click(VKey::Num0),
        79 => click(VKey::Num5),
        80 => click(VKey::Np8),
        81 => click(VKey::Num6),
        82 => click(VKey::Np9),
        83 => click(VKey::Num7),

        84 => click(VKey::F1),
        85 => click(VKey::F8),
        86 => click(VKey::F2),
        87 => click(VKey::F9),
        88 => click(VKey::F3),
        89 => click(VKey::F4),
        90 => click(VKey::F10),
        91 => click(VKey::F5),
        92 => click(VKey::NpDivide),
        93 => click(VKey::F6),
        94 => click(VKey::NpMultiply),
        95 => click(VKey::F7),
        _ => {}
    };
}

#[inline]
pub fn delta_force(val: i32, is_on: bool) {
    let (vk, mice) = match note_to_key_and_mice(val) {
        Some(res) => res,
        None => return,
    };

    const ALL_MODIFIERS: [MouseButton; 3] =
        [MouseButton::Left, MouseButton::Right, MouseButton::Middle];

    if is_on {
        for &mb in &ALL_MODIFIERS {
            if !mice.contains(&mb) {
                mouse_release(mb);
            }
        }

        for &mb in mice {
            mouse_press(mb);
        }

        handle_key(vk, true);
    } else {
        handle_key(vk, false);
        for &mb in mice.iter().rev() {
            mouse_release(mb);
        }
    }
}

/// 将 MIDI 音符值解析为对应的键盘按键与鼠标变调组合
fn note_to_key_and_mice(val: i32) -> Option<(VKey, &'static [MouseButton])> {
    const M_LEFT: &[MouseButton] = &[MouseButton::Left];
    const M_LEFT_MID: &[MouseButton] = &[MouseButton::Left, MouseButton::Middle];
    const M_NONE: &[MouseButton] = &[];
    const M_MID: &[MouseButton] = &[MouseButton::Middle];
    const M_RIGHT: &[MouseButton] = &[MouseButton::Right];
    const M_RIGHT_MID: &[MouseButton] = &[MouseButton::Right, MouseButton::Middle];

    let res = match val {
        // 低音区 (48~59 / C3~B3) -> Left 键变调
        48 => (VKey::Z, M_LEFT),
        49 => (VKey::Z, M_LEFT_MID),
        50 => (VKey::X, M_LEFT),
        51 => (VKey::X, M_LEFT_MID),
        52 => (VKey::C, M_LEFT),
        53 => (VKey::V, M_LEFT),
        54 => (VKey::V, M_LEFT_MID),
        55 => (VKey::B, M_LEFT),
        56 => (VKey::B, M_LEFT_MID),
        57 => (VKey::N, M_LEFT),
        58 => (VKey::N, M_LEFT_MID),
        59 => (VKey::M, M_LEFT),

        // 中音区 (60~71 / C4~B4) -> 无 Octave 变调 (必要时仅保留 Middle 升半音)
        60 => (VKey::Z, M_NONE),
        61 => (VKey::Z, M_MID),
        62 => (VKey::X, M_NONE),
        63 => (VKey::X, M_MID),
        64 => (VKey::C, M_NONE),
        65 => (VKey::V, M_NONE),
        66 => (VKey::V, M_MID),
        67 => (VKey::B, M_NONE),
        68 => (VKey::B, M_MID),
        69 => (VKey::N, M_NONE),
        70 => (VKey::N, M_MID),
        71 => (VKey::M, M_NONE),

        // 高音区 (72~83 / C5~B5) -> Right 键变调
        72 => (VKey::Z, M_RIGHT),
        73 => (VKey::Z, M_RIGHT_MID),
        74 => (VKey::X, M_RIGHT),
        75 => (VKey::X, M_RIGHT_MID),
        76 => (VKey::C, M_RIGHT),
        77 => (VKey::V, M_RIGHT),
        78 => (VKey::V, M_RIGHT_MID),
        79 => (VKey::B, M_RIGHT),
        80 => (VKey::B, M_RIGHT_MID),
        81 => (VKey::N, M_RIGHT),
        82 => (VKey::N, M_RIGHT_MID),
        83 => (VKey::M, M_RIGHT),

        // 倍高音区 (84~85 / C6~C#6)
        84 => (VKey::Comma, M_RIGHT),
        85 => (VKey::Comma, M_RIGHT_MID),

        _ => return None,
    };

    Some(res)
}

#[inline]
fn key_event(vk: VKey, flags: u32) {
    unsafe {
        let input = [INPUT {
            type_: INPUT_KEYBOARD,
            u: {
                let mut u = mem::zeroed::<INPUT_u>();
                *u.ki_mut() = KEYBDINPUT {
                    wVk: 0,
                    wScan: MapVirtualKeyA(vk as _, MAPVK_VK_TO_VSC) as _,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                };
                u
            },
        }];

        SendInput(
            input.len() as _,
            input.as_ptr() as _,
            size_of::<INPUT>() as _,
        );
    }
}

#[inline]
fn press(vk: VKey) {
    key_event(vk, KEYEVENTF_SCANCODE);
}

#[inline]
fn release(vk: VKey) {
    key_event(vk, KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP);
}

#[inline]
#[allow(dead_code)]
fn click(vk: VKey) {
    press(vk);
    release(vk);
}

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseButton {
    #[inline]
    fn down_flag(self) -> u32 {
        match self {
            MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
            MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN,
        }
    }

    #[inline]
    fn up_flag(self) -> u32 {
        match self {
            MouseButton::Left => MOUSEEVENTF_LEFTUP,
            MouseButton::Right => MOUSEEVENTF_RIGHTUP,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
        }
    }
}

#[inline]
fn mouse_event(flags: u32) {
    unsafe {
        let input = [INPUT {
            type_: INPUT_MOUSE,
            u: {
                let mut u = mem::zeroed::<INPUT_u>();

                *u.mi_mut() = MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                };

                u
            },
        }];

        SendInput(
            input.len() as _,
            input.as_ptr() as _,
            size_of::<INPUT>() as _,
        );
    }
}

#[inline]
pub fn mouse_press(button: MouseButton) {
    mouse_event(button.down_flag());
}

#[inline]
pub fn mouse_release(button: MouseButton) {
    mouse_event(button.up_flag());
}

pub fn release_all() {
    MouseButton::iter().for_each(|it| mouse_release(it));
}
