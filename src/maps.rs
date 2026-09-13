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
    // 定义辅助闭包：处理带有鼠标修饰键的复合按键
    let dispatch = |vk: VKey, mouse: Option<MouseButton>| {
        if is_on {
            if let Some(mb) = mouse {
                mouse_press(mb);
            }
            handle_key(vk, true);
        } else {
            handle_key(vk, false);
            if let Some(mb) = mouse {
                mouse_release(mb);
            }
        }
    };

    match val {
        // ================= 低音区 (降调: 鼠标左键 Left) =================
        48 => dispatch(VKey::Z, Some(MouseButton::Left)), // 低音 1 (C3)
        49 => dispatch(VKey::Z, Some(MouseButton::Left)), // 低音 #1 (C#3) -> 组合鼠标中键可按需增强，若游戏中只靠降调按键则映射至主按键
        50 => dispatch(VKey::X, Some(MouseButton::Left)), // 低音 2 (D3)
        51 => dispatch(VKey::X, Some(MouseButton::Left)), // 低音 #2 (D#3)
        52 => dispatch(VKey::C, Some(MouseButton::Left)), // 低音 3 (E3)
        53 => dispatch(VKey::V, Some(MouseButton::Left)), // 低音 4 (F3)
        54 => dispatch(VKey::V, Some(MouseButton::Left)), // 低音 #4 (F#3)
        55 => dispatch(VKey::B, Some(MouseButton::Left)), // 低音 5 (G3)
        56 => dispatch(VKey::B, Some(MouseButton::Left)), // 低音 #5 (G#3)
        57 => dispatch(VKey::N, Some(MouseButton::Left)), // 低音 6 (A3)
        58 => dispatch(VKey::N, Some(MouseButton::Left)), // 低音 #6 (A#3)
        59 => dispatch(VKey::M, Some(MouseButton::Left)), // 低音 7 (B3)

        // ================= 中音区 (自然音: 无鼠标修饰) =================
        60 => dispatch(VKey::Z, None), // 中音 1 (C4)
        61 => dispatch(VKey::Z, Some(MouseButton::Middle)), // 中音 #1 (C#4) -> 半音
        62 => dispatch(VKey::X, None), // 中音 2 (D4)
        63 => dispatch(VKey::X, Some(MouseButton::Middle)), // 中音 #2 (D#4) -> 半音
        64 => dispatch(VKey::C, None), // 中音 3 (E4)
        65 => dispatch(VKey::V, None), // 中音 4 (F4)
        66 => dispatch(VKey::V, Some(MouseButton::Middle)), // 中音 #4 (F#4) -> 半音
        67 => dispatch(VKey::B, None), // 中音 5 (G4)
        68 => dispatch(VKey::B, Some(MouseButton::Middle)), // 中音 #5 (G#4) -> 半音
        69 => dispatch(VKey::N, None), // 中音 6 (A4)
        70 => dispatch(VKey::N, Some(MouseButton::Middle)), // 中音 #6 (A#4) -> 半音
        71 => dispatch(VKey::M, None), // 中音 7 (B4)

        // ================= 高音区 (升调: 鼠标右键 Right) =================
        72 => dispatch(VKey::Comma, None), // 高音 1 (C5) -> 原图中的逗号键
        73 => dispatch(VKey::Z, Some(MouseButton::Right)), // 高音 #1 (C#5)
        74 => dispatch(VKey::X, Some(MouseButton::Right)), // 高音 2 (D5)
        75 => dispatch(VKey::X, Some(MouseButton::Right)), // 高音 #2 (D#5)
        76 => dispatch(VKey::C, Some(MouseButton::Right)), // 高音 3 (E5)
        77 => dispatch(VKey::V, Some(MouseButton::Right)), // 高音 4 (F5)
        78 => dispatch(VKey::V, Some(MouseButton::Right)), // 高音 #4 (F#5)
        79 => dispatch(VKey::B, Some(MouseButton::Right)), // 高音 5 (G5)
        80 => dispatch(VKey::B, Some(MouseButton::Right)), // 高音 #5 (G#5)
        81 => dispatch(VKey::N, Some(MouseButton::Right)), // 高音 6 (A5)
        82 => dispatch(VKey::N, Some(MouseButton::Right)), // 高音 #6 (A#5)
        83 => dispatch(VKey::M, Some(MouseButton::Right)), // 高音 7 (B5)
        84 => dispatch(VKey::Comma, Some(MouseButton::Right)), // 倍高音 1 (C6)

        _ => {}
    }
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

#[derive(Clone, Copy, EnumIter)]
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
    let keys = [
        // 字母键 (来自 MAP 数组以及 VRChat / DeltaForce 映射)
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
        VKey::K,
        VKey::L,
        VKey::Z,
        VKey::X,
        VKey::C,
        VKey::V,
        VKey::B,
        VKey::N,
        VKey::M,
        VKey::I,
        VKey::O,
        VKey::P,
        // 标点符号键
        VKey::Comma,
        VKey::Period,
        VKey::Slash,
        VKey::Semicolon,
        // 主键盘数字键
        VKey::Num0,
        VKey::Num1,
        VKey::Num2,
        VKey::Num3,
        VKey::Num4,
        VKey::Num5,
        VKey::Num6,
        VKey::Num7,
        VKey::Num8,
        VKey::Num9,
        // 功能键 (F1 - F10)
        VKey::F1,
        VKey::F2,
        VKey::F3,
        VKey::F4,
        VKey::F5,
        VKey::F6,
        VKey::F7,
        VKey::F8,
        VKey::F9,
        VKey::F10,
        // 小键盘按键
        VKey::Np0,
        VKey::Np2,
        VKey::Np3,
        VKey::Np5,
        VKey::Np6,
        VKey::Np8,
        VKey::Np9,
        VKey::NpDecimal,
        VKey::NpDivide,
        VKey::NpMultiply,
    ];

    for vk in keys {
        release(vk);
    }
}
