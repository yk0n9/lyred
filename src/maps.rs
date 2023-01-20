use enigo::Key;
use enigo::Key::Raw;

pub const GEN_SHIN: fn(u8) -> Option<Key> = |key| -> Option<Key> {
    Some(match key {
        24 => Key::Layout('z'),
        26 => Key::Layout('x'),
        28 => Key::Layout('c'),
        29 => Key::Layout('v'),
        31 => Key::Layout('b'),
        33 => Key::Layout('n'),
        35 => Key::Layout('m'),
        36 => Key::Layout('z'),
        38 => Key::Layout('x'),
        40 => Key::Layout('c'),
        41 => Key::Layout('v'),
        43 => Key::Layout('b'),
        45 => Key::Layout('n'),
        47 => Key::Layout('m'),
        48 => Key::Layout('z'),
        50 => Key::Layout('x'),
        52 => Key::Layout('c'),
        53 => Key::Layout('v'),
        55 => Key::Layout('b'),
        57 => Key::Layout('n'),
        59 => Key::Layout('m'),
        60 => Key::Layout('a'),
        62 => Key::Layout('s'),
        64 => Key::Layout('d'),
        65 => Key::Layout('f'),
        67 => Key::Layout('g'),
        69 => Key::Layout('h'),
        71 => Key::Layout('j'),
        72 => Key::Layout('q'),
        74 => Key::Layout('w'),
        76 => Key::Layout('e'),
        77 => Key::Layout('r'),
        79 => Key::Layout('t'),
        81 => Key::Layout('y'),
        83 => Key::Layout('u'),
        84 => Key::Layout('q'),
        86 => Key::Layout('w'),
        88 => Key::Layout('e'),
        89 => Key::Layout('r'),
        91 => Key::Layout('t'),
        93 => Key::Layout('y'),
        95 => Key::Layout('u'),
        _ => return None,
    })
};

pub const VR_CHAT: fn(u8) -> Option<Key> = |key| -> Option<Key> {
    Some(match key {
        36 => Raw(90),
        37 => Raw(188),
        38 => Raw(88),
        39 => Raw(190),
        40 => Raw(67),
        41 => Raw(86),
        42 => Raw(191),
        43 => Raw(66),
        44 => Raw(96),
        45 => Raw(78),
        46 => Raw(110),
        47 => Raw(77),
        48 => Raw(65),
        49 => Raw(75),
        50 => Raw(83),
        51 => Raw(76),
        52 => Raw(68),
        53 => Raw(70),
        54 => Raw(186),
        55 => Raw(71),
        56 => Raw(98),
        57 => Raw(72),
        58 => Raw(99),
        59 => Raw(74),
        60 => Raw(81),
        61 => Raw(73),
        62 => Raw(87),
        63 => Raw(79),
        64 => Raw(69),
        65 => Raw(82),
        66 => Raw(80),
        67 => Raw(84),
        68 => Raw(101),
        69 => Raw(89),
        70 => Raw(102),
        71 => Raw(85),
        72 => Raw(49),
        73 => Raw(56),
        74 => Raw(50),
        75 => Raw(57),
        76 => Raw(51),
        77 => Raw(52),
        78 => Raw(48),
        79 => Raw(53),
        80 => Raw(104),
        81 => Raw(54),
        82 => Raw(105),
        83 => Raw(55),
        84 => Raw(112),
        85 => Raw(119),
        86 => Raw(113),
        87 => Raw(120),
        88 => Raw(114),
        89 => Raw(115),
        90 => Raw(121),
        91 => Raw(116),
        92 => Raw(111),
        93 => Raw(117),
        94 => Raw(106),
        95 => Raw(118),
        _ => return None
    })
};