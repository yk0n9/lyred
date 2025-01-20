use std::mem;

use winapi::um::winuser::{
    GetAsyncKeyState, INPUT_u, MapVirtualKeyA, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
    MAPVK_VK_TO_VSC,
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
pub fn get_map(mode: Mode) -> impl Fn(i32) {
    match mode {
        Mode::GenShin => gen_shin,
        Mode::VRChat => vr_chat,
    }
}

#[inline]
pub fn gen_shin(val: i32) {
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
pub fn vr_chat(val: i32) {
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
fn click(vk: VKey) {
    unsafe {
        let mut inputs = [
            INPUT {
                type_: INPUT_KEYBOARD,
                u: {
                    let mut u = mem::zeroed::<INPUT_u>();
                    *u.ki_mut() = KEYBDINPUT {
                        wVk: vk as _,
                        wScan: MapVirtualKeyA(vk as _, MAPVK_VK_TO_VSC) as _,
                        dwFlags: 0,
                        time: 0,
                        dwExtraInfo: 0,
                    };
                    u
                },
            },
            INPUT {
                type_: INPUT_KEYBOARD,
                u: {
                    let mut u = mem::zeroed::<INPUT_u>();
                    *u.ki_mut() = KEYBDINPUT {
                        wVk: vk as _,
                        wScan: MapVirtualKeyA(vk as _, MAPVK_VK_TO_VSC) as _,
                        dwFlags: 2,
                        time: 0,
                        dwExtraInfo: 0,
                    };
                    u
                },
            },
        ];
        SendInput(
            inputs.len() as _,
            inputs.as_mut_ptr(),
            mem::size_of::<INPUT>() as _,
        );
    }
}
