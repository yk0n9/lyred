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

#[inline(always)]
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

#[inline(always)]
pub fn vr_chat(val: i32) {
    match val {
        36 => click(90),
        37 => click(188),
        38 => click(88),
        39 => click(190),
        40 => click(67),
        41 => click(86),
        42 => click(191),
        43 => click(66),
        44 => click(96),
        45 => click(78),
        46 => click(110),
        47 => click(77),
        48 => click(65),
        49 => click(75),
        50 => click(83),
        51 => click(76),
        52 => click(68),
        53 => click(70),
        54 => click(186),
        55 => click(71),
        56 => click(98),
        57 => click(72),
        58 => click(99),
        59 => click(74),
        60 => click(81),
        61 => click(73),
        62 => click(87),
        63 => click(79),
        64 => click(69),
        65 => click(82),
        66 => click(80),
        67 => click(84),
        68 => click(101),
        69 => click(89),
        70 => click(102),
        71 => click(85),
        72 => click(49),
        73 => click(56),
        74 => click(50),
        75 => click(57),
        76 => click(51),
        77 => click(52),
        78 => click(48),
        79 => click(53),
        80 => click(104),
        81 => click(54),
        82 => click(105),
        83 => click(55),
        84 => click(112),
        85 => click(119),
        86 => click(113),
        87 => click(120),
        88 => click(114),
        89 => click(115),
        90 => click(121),
        91 => click(116),
        92 => click(111),
        93 => click(117),
        94 => click(106),
        95 => click(118),
        _ => {}
    };
}

#[inline(always)]
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
