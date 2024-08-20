use std::fs;

use rfd::MessageButtons;

use crate::midi::{Event, Midi};
use crate::POOL;

impl Midi {
    pub fn convert_from_midi(self, name: String) {
        POOL.spawn(move || {
            let mut res = String::new();
            let mut cache = String::new();
            let mut count = 0;

            self.events.read().iter().for_each(|event| {
                if event.delay != 0.0 {
                    if !cache.is_empty() {
                        let s = if count > 1 {
                            format!("[{cache}] ")
                        } else {
                            format!("{cache} ")
                        };
                        res.push_str(&s);
                    }
                    cache = format!("{} ", event.get_key());
                    count = 1;
                } else {
                    cache.push_str(&format!("{} ", event.get_key()));
                    count += 1;
                }
            });

            let phone = res
                .replace('q', "+1")
                .replace('w', "+2")
                .replace('e', "+3")
                .replace('r', "+4")
                .replace('t', "+5")
                .replace('y', "+6")
                .replace('u', "+7")
                .replace('a', "1")
                .replace('s', "2")
                .replace('d', "3")
                .replace('f', "4")
                .replace('g', "5")
                .replace('h', "6")
                .replace('j', "7")
                .replace('z', "-1")
                .replace('x', "-2")
                .replace('c', "-3")
                .replace('v', "-4")
                .replace('b', "-5")
                .replace('n', "-6")
                .replace('m', "-7");
            if let (Ok(_), Ok(_)) = (
                fs::write(format!("{}.txt", name), res.to_uppercase().as_bytes()),
                fs::write(format!("phone-{}.txt", name), phone.as_bytes()),
            ) {
                rfd::MessageDialog::new()
                    .set_description("转换成功\n请查看当前目录下的txt文本文件")
                    .set_buttons(MessageButtons::Ok)
                    .show();
            } else {
                rfd::MessageDialog::new()
                    .set_description("转换失败")
                    .set_buttons(MessageButtons::Ok)
                    .show();
            }
        });
    }
}

impl Event {
    fn get_key(&self) -> &'static str {
        match self.press {
            24 => "z",
            26 => "x",
            28 => "c",
            29 => "v",
            31 => "b",
            33 => "n",
            35 => "m",
            36 => "z",
            38 => "x",
            40 => "c",
            41 => "v",
            43 => "b",
            45 => "n",
            47 => "m",
            48 => "z",
            50 => "x",
            52 => "c",
            53 => "v",
            55 => "b",
            57 => "n",
            59 => "m",
            60 => "a",
            62 => "s",
            64 => "d",
            65 => "f",
            67 => "g",
            69 => "h",
            71 => "j",
            72 => "q",
            74 => "w",
            76 => "e",
            77 => "r",
            79 => "t",
            81 => "y",
            83 => "u",
            84 => "q",
            86 => "w",
            88 => "e",
            89 => "r",
            91 => "t",
            93 => "y",
            95 => "u",
            _ => "",
        }
    }
}
