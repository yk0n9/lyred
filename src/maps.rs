use std::mem::size_of;

use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::ui::play::Mode;

#[inline]
pub fn is_pressed(vk: u16) -> bool {
    let status = unsafe { GetAsyncKeyState(vk as i32) as u32 };
    status >> 31 == 1
}

#[inline]
pub fn get_map(mode: Mode) -> impl Fn(i32) {
    match mode {
        Mode::GenShin => gen_shin,
        Mode::VRChat => vr_chat,
    }
}

#[inline(always)]
pub fn gen_shin(key: i32) {
    match key {
        24 => click(VK_Z),
        26 => click(VK_X),
        28 => click(VK_C),
        29 => click(VK_V),
        31 => click(VK_B),
        33 => click(VK_N),
        35 => click(VK_M),
        36 => click(VK_Z),
        38 => click(VK_X),
        40 => click(VK_C),
        41 => click(VK_V),
        43 => click(VK_B),
        45 => click(VK_N),
        47 => click(VK_M),
        48 => click(VK_Z),
        50 => click(VK_X),
        52 => click(VK_C),
        53 => click(VK_V),
        55 => click(VK_B),
        57 => click(VK_N),
        59 => click(VK_M),
        60 => click(VK_A),
        62 => click(VK_S),
        64 => click(VK_D),
        65 => click(VK_F),
        67 => click(VK_G),
        69 => click(VK_H),
        71 => click(VK_J),
        72 => click(VK_Q),
        74 => click(VK_W),
        76 => click(VK_E),
        77 => click(VK_R),
        79 => click(VK_T),
        81 => click(VK_Y),
        83 => click(VK_U),
        84 => click(VK_Q),
        86 => click(VK_W),
        88 => click(VK_E),
        89 => click(VK_R),
        91 => click(VK_T),
        93 => click(VK_Y),
        95 => click(VK_U),
        _ => {}
    };
}

#[inline(always)]
pub fn vr_chat(key: i32) {
    match key {
        36 => click(VIRTUAL_KEY(90)),
        37 => click(VIRTUAL_KEY(188)),
        38 => click(VIRTUAL_KEY(88)),
        39 => click(VIRTUAL_KEY(190)),
        40 => click(VIRTUAL_KEY(67)),
        41 => click(VIRTUAL_KEY(86)),
        42 => click(VIRTUAL_KEY(191)),
        43 => click(VIRTUAL_KEY(66)),
        44 => click(VIRTUAL_KEY(96)),
        45 => click(VIRTUAL_KEY(78)),
        46 => click(VIRTUAL_KEY(110)),
        47 => click(VIRTUAL_KEY(77)),
        48 => click(VIRTUAL_KEY(65)),
        49 => click(VIRTUAL_KEY(75)),
        50 => click(VIRTUAL_KEY(83)),
        51 => click(VIRTUAL_KEY(76)),
        52 => click(VIRTUAL_KEY(68)),
        53 => click(VIRTUAL_KEY(70)),
        54 => click(VIRTUAL_KEY(186)),
        55 => click(VIRTUAL_KEY(71)),
        56 => click(VIRTUAL_KEY(98)),
        57 => click(VIRTUAL_KEY(72)),
        58 => click(VIRTUAL_KEY(99)),
        59 => click(VIRTUAL_KEY(74)),
        60 => click(VIRTUAL_KEY(81)),
        61 => click(VIRTUAL_KEY(73)),
        62 => click(VIRTUAL_KEY(87)),
        63 => click(VIRTUAL_KEY(79)),
        64 => click(VIRTUAL_KEY(69)),
        65 => click(VIRTUAL_KEY(82)),
        66 => click(VIRTUAL_KEY(80)),
        67 => click(VIRTUAL_KEY(84)),
        68 => click(VIRTUAL_KEY(101)),
        69 => click(VIRTUAL_KEY(89)),
        70 => click(VIRTUAL_KEY(102)),
        71 => click(VIRTUAL_KEY(85)),
        72 => click(VIRTUAL_KEY(49)),
        73 => click(VIRTUAL_KEY(56)),
        74 => click(VIRTUAL_KEY(50)),
        75 => click(VIRTUAL_KEY(57)),
        76 => click(VIRTUAL_KEY(51)),
        77 => click(VIRTUAL_KEY(52)),
        78 => click(VIRTUAL_KEY(48)),
        79 => click(VIRTUAL_KEY(53)),
        80 => click(VIRTUAL_KEY(104)),
        81 => click(VIRTUAL_KEY(54)),
        82 => click(VIRTUAL_KEY(105)),
        83 => click(VIRTUAL_KEY(55)),
        84 => click(VIRTUAL_KEY(112)),
        85 => click(VIRTUAL_KEY(119)),
        86 => click(VIRTUAL_KEY(113)),
        87 => click(VIRTUAL_KEY(120)),
        88 => click(VIRTUAL_KEY(114)),
        89 => click(VIRTUAL_KEY(115)),
        90 => click(VIRTUAL_KEY(121)),
        91 => click(VIRTUAL_KEY(116)),
        92 => click(VIRTUAL_KEY(111)),
        93 => click(VIRTUAL_KEY(117)),
        94 => click(VIRTUAL_KEY(106)),
        95 => click(VIRTUAL_KEY(118)),
        _ => {}
    };
}

const SIZE: i32 = size_of::<INPUT>() as i32;

#[inline(always)]
fn click(vk: VIRTUAL_KEY) {
    unsafe {
        SendInput(
            &[
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: vk,
                            wScan: MapVirtualKeyA(vk.0 as u32, MAPVK_VK_TO_VSC) as u16,
                            dwFlags: KEYBD_EVENT_FLAGS(0),
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: vk,
                            wScan: MapVirtualKeyA(vk.0 as u32, MAPVK_VK_TO_VSC) as u16,
                            dwFlags: KEYBD_EVENT_FLAGS(2),
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ],
            SIZE,
        );
    }
}
