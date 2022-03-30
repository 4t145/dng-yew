use crate::{components::drawpad::Color, rgb};

pub fn parse_color<'a>(s: impl Into<&'a str>) -> Option<Color> {
    let s:&str = s.into();
    if s.len() == 7 && s.starts_with('#') {
        let number = &s[1..7];
        if let Ok(code) = u32::from_str_radix(number, 16) {
            return Some(rgb!((code>>16)&0xff,(code>>8)&0xff,(code)&0xff));
        }
    }
    None
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(format!($($arg)*).as_str()));
    };
}