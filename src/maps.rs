use crate::ui::play::Mode;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[inline]
pub fn is_pressed(vk: u16) -> bool {
    let status = unsafe { GetAsyncKeyState(vk as i32) as u32 };
    (status >> 31) == 1
}

#[inline(always)]
fn send(vk: VIRTUAL_KEY) {
    press(vk);
    release(vk);
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
        24 => send(VK_Z),
        26 => send(VK_X),
        28 => send(VK_C),
        29 => send(VK_V),
        31 => send(VK_B),
        33 => send(VK_N),
        35 => send(VK_M),
        36 => send(VK_Z),
        38 => send(VK_X),
        40 => send(VK_C),
        41 => send(VK_V),
        43 => send(VK_B),
        45 => send(VK_N),
        47 => send(VK_M),
        48 => send(VK_Z),
        50 => send(VK_X),
        52 => send(VK_C),
        53 => send(VK_V),
        55 => send(VK_B),
        57 => send(VK_N),
        59 => send(VK_M),
        60 => send(VK_A),
        62 => send(VK_S),
        64 => send(VK_D),
        65 => send(VK_F),
        67 => send(VK_G),
        69 => send(VK_H),
        71 => send(VK_J),
        72 => send(VK_Q),
        74 => send(VK_W),
        76 => send(VK_E),
        77 => send(VK_R),
        79 => send(VK_T),
        81 => send(VK_Y),
        83 => send(VK_U),
        84 => send(VK_Q),
        86 => send(VK_W),
        88 => send(VK_E),
        89 => send(VK_R),
        91 => send(VK_T),
        93 => send(VK_Y),
        95 => send(VK_U),
        _ => {}
    };
}

#[inline(always)]
pub fn vr_chat(key: i32) {
    match key {
        36 => send(VIRTUAL_KEY(90)),
        37 => send(VIRTUAL_KEY(188)),
        38 => send(VIRTUAL_KEY(88)),
        39 => send(VIRTUAL_KEY(190)),
        40 => send(VIRTUAL_KEY(67)),
        41 => send(VIRTUAL_KEY(86)),
        42 => send(VIRTUAL_KEY(191)),
        43 => send(VIRTUAL_KEY(66)),
        44 => send(VIRTUAL_KEY(96)),
        45 => send(VIRTUAL_KEY(78)),
        46 => send(VIRTUAL_KEY(110)),
        47 => send(VIRTUAL_KEY(77)),
        48 => send(VIRTUAL_KEY(65)),
        49 => send(VIRTUAL_KEY(75)),
        50 => send(VIRTUAL_KEY(83)),
        51 => send(VIRTUAL_KEY(76)),
        52 => send(VIRTUAL_KEY(68)),
        53 => send(VIRTUAL_KEY(70)),
        54 => send(VIRTUAL_KEY(186)),
        55 => send(VIRTUAL_KEY(71)),
        56 => send(VIRTUAL_KEY(98)),
        57 => send(VIRTUAL_KEY(72)),
        58 => send(VIRTUAL_KEY(99)),
        59 => send(VIRTUAL_KEY(74)),
        60 => send(VIRTUAL_KEY(81)),
        61 => send(VIRTUAL_KEY(73)),
        62 => send(VIRTUAL_KEY(87)),
        63 => send(VIRTUAL_KEY(79)),
        64 => send(VIRTUAL_KEY(69)),
        65 => send(VIRTUAL_KEY(82)),
        66 => send(VIRTUAL_KEY(80)),
        67 => send(VIRTUAL_KEY(84)),
        68 => send(VIRTUAL_KEY(101)),
        69 => send(VIRTUAL_KEY(89)),
        70 => send(VIRTUAL_KEY(102)),
        71 => send(VIRTUAL_KEY(85)),
        72 => send(VIRTUAL_KEY(49)),
        73 => send(VIRTUAL_KEY(56)),
        74 => send(VIRTUAL_KEY(50)),
        75 => send(VIRTUAL_KEY(57)),
        76 => send(VIRTUAL_KEY(51)),
        77 => send(VIRTUAL_KEY(52)),
        78 => send(VIRTUAL_KEY(48)),
        79 => send(VIRTUAL_KEY(53)),
        80 => send(VIRTUAL_KEY(104)),
        81 => send(VIRTUAL_KEY(54)),
        82 => send(VIRTUAL_KEY(105)),
        83 => send(VIRTUAL_KEY(55)),
        84 => send(VIRTUAL_KEY(112)),
        85 => send(VIRTUAL_KEY(119)),
        86 => send(VIRTUAL_KEY(113)),
        87 => send(VIRTUAL_KEY(120)),
        88 => send(VIRTUAL_KEY(114)),
        89 => send(VIRTUAL_KEY(115)),
        90 => send(VIRTUAL_KEY(121)),
        91 => send(VIRTUAL_KEY(116)),
        92 => send(VIRTUAL_KEY(111)),
        93 => send(VIRTUAL_KEY(117)),
        94 => send(VIRTUAL_KEY(106)),
        95 => send(VIRTUAL_KEY(118)),
        _ => {}
    };
}

#[inline(always)]
fn press(vk: VIRTUAL_KEY) {
    unsafe {
        let mut input = std::mem::zeroed::<INPUT>();
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki = KEYBDINPUT {
            wVk: vk,
            wScan: MapVirtualKeyA(vk.0 as u32, MAPVK_VK_TO_VSC) as u16,
            dwFlags: KEYBD_EVENT_FLAGS(0),
            time: 0,
            dwExtraInfo: 0,
        };
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

#[inline(always)]
fn release(vk: VIRTUAL_KEY) {
    unsafe {
        let mut input = std::mem::zeroed::<INPUT>();
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki = KEYBDINPUT {
            wVk: vk,
            wScan: MapVirtualKeyA(vk.0 as u32, MAPVK_VK_TO_VSC) as u16,
            dwFlags: KEYBD_EVENT_FLAGS(2),
            time: 0,
            dwExtraInfo: 0,
        };
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}
