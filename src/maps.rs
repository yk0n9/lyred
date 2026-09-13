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
    unsafe {
        match val {
            24 => handle_key(MAP[14], is_on),
            26 => handle_key(MAP[15], is_on),
            28 => handle_key(MAP[16], is_on),
            29 => handle_key(MAP[17], is_on),
            31 => handle_key(MAP[18], is_on),
            33 => handle_key(MAP[19], is_on),
            35 => handle_key(MAP[20], is_on),
            36 => handle_key(MAP[14], is_on),
            38 => handle_key(MAP[15], is_on),
            40 => handle_key(MAP[16], is_on),
            41 => handle_key(MAP[17], is_on),
            43 => handle_key(MAP[18], is_on),
            45 => handle_key(MAP[19], is_on),
            47 => handle_key(MAP[20], is_on),

            48 => handle_key(MAP[14], is_on),
            50 => handle_key(MAP[15], is_on),
            52 => handle_key(MAP[16], is_on),
            53 => handle_key(MAP[17], is_on),
            55 => handle_key(MAP[18], is_on),
            57 => handle_key(MAP[19], is_on),
            59 => handle_key(MAP[20], is_on),

            60 => handle_key(MAP[7], is_on),
            62 => handle_key(MAP[8], is_on),
            64 => handle_key(MAP[9], is_on),
            65 => handle_key(MAP[10], is_on),
            67 => handle_key(MAP[11], is_on),
            69 => handle_key(MAP[12], is_on),
            71 => handle_key(MAP[13], is_on),

            72 => handle_key(MAP[0], is_on),
            74 => handle_key(MAP[1], is_on),
            76 => handle_key(MAP[2], is_on),
            77 => handle_key(MAP[3], is_on),
            79 => handle_key(MAP[4], is_on),
            81 => handle_key(MAP[5], is_on),
            83 => handle_key(MAP[6], is_on),
            84 => handle_key(MAP[0], is_on),
            86 => handle_key(MAP[1], is_on),
            88 => handle_key(MAP[2], is_on),
            89 => handle_key(MAP[3], is_on),
            91 => handle_key(MAP[4], is_on),
            93 => handle_key(MAP[5], is_on),
            95 => handle_key(MAP[6], is_on),
            _ => {}
        };
    }
}

#[inline]
pub fn vr_chat(val: i32, is_on: bool) {
    match val {
        36 => handle_key(VKey::Z, is_on),
        37 => handle_key(VKey::Comma, is_on),
        38 => handle_key(VKey::X, is_on),
        39 => handle_key(VKey::Period, is_on),
        40 => handle_key(VKey::C, is_on),
        41 => handle_key(VKey::V, is_on),
        42 => handle_key(VKey::Slash, is_on),
        43 => handle_key(VKey::B, is_on),
        44 => handle_key(VKey::Np0, is_on),
        45 => handle_key(VKey::N, is_on),
        46 => handle_key(VKey::NpDecimal, is_on),
        47 => handle_key(VKey::M, is_on),

        48 => handle_key(VKey::A, is_on),
        49 => handle_key(VKey::K, is_on),
        50 => handle_key(VKey::S, is_on),
        51 => handle_key(VKey::L, is_on),
        52 => handle_key(VKey::D, is_on),
        53 => handle_key(VKey::F, is_on),
        54 => handle_key(VKey::Semicolon, is_on),
        55 => handle_key(VKey::G, is_on),
        56 => handle_key(VKey::Np2, is_on),
        57 => handle_key(VKey::H, is_on),
        58 => handle_key(VKey::Np3, is_on),
        59 => handle_key(VKey::J, is_on),

        60 => handle_key(VKey::Q, is_on),
        61 => handle_key(VKey::I, is_on),
        62 => handle_key(VKey::W, is_on),
        63 => handle_key(VKey::O, is_on),
        64 => handle_key(VKey::E, is_on),
        65 => handle_key(VKey::R, is_on),
        66 => handle_key(VKey::P, is_on),
        67 => handle_key(VKey::T, is_on),
        68 => handle_key(VKey::Np5, is_on),
        69 => handle_key(VKey::Y, is_on),
        70 => handle_key(VKey::Np6, is_on),
        71 => handle_key(VKey::U, is_on),

        72 => handle_key(VKey::Num1, is_on),
        73 => handle_key(VKey::Num8, is_on),
        74 => handle_key(VKey::Num2, is_on),
        75 => handle_key(VKey::Num9, is_on),
        76 => handle_key(VKey::Num3, is_on),
        77 => handle_key(VKey::Num4, is_on),
        78 => handle_key(VKey::Num0, is_on),
        79 => handle_key(VKey::Num5, is_on),
        80 => handle_key(VKey::Np8, is_on),
        81 => handle_key(VKey::Num6, is_on),
        82 => handle_key(VKey::Np9, is_on),
        83 => handle_key(VKey::Num7, is_on),

        84 => handle_key(VKey::F1, is_on),
        85 => handle_key(VKey::F8, is_on),
        86 => handle_key(VKey::F2, is_on),
        87 => handle_key(VKey::F9, is_on),
        88 => handle_key(VKey::F3, is_on),
        89 => handle_key(VKey::F4, is_on),
        90 => handle_key(VKey::F10, is_on),
        91 => handle_key(VKey::F5, is_on),
        92 => handle_key(VKey::NpDivide, is_on),
        93 => handle_key(VKey::F6, is_on),
        94 => handle_key(VKey::NpMultiply, is_on),
        95 => handle_key(VKey::F7, is_on),
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

fn press(vk: VKey) {
    unsafe {
        let input = &[INPUT {
            type_: INPUT_KEYBOARD,
            u: {
                let mut u = mem::zeroed::<INPUT_u>();
                *u.ki_mut() = KEYBDINPUT {
                    wVk: 0,
                    wScan: MapVirtualKeyA(vk as _, MAPVK_VK_TO_VSC) as _,
                    dwFlags: KEYEVENTF_SCANCODE,
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

fn release(vk: VKey) {
    unsafe {
        let input = &[INPUT {
            type_: INPUT_KEYBOARD,
            u: {
                let mut u = mem::zeroed::<INPUT_u>();
                *u.ki_mut() = KEYBDINPUT {
                    wVk: 0,
                    wScan: MapVirtualKeyA(vk as _, MAPVK_VK_TO_VSC) as _,
                    dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
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
