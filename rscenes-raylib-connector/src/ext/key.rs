use crate::rcore::Rcore;
use raylib_ffi::enums::KeyboardKey;
use std::char;

pub trait KeyboardKeyExt: Sized {
    fn get_pressed() -> Self;
    fn get_pressed_char() -> String;
    fn to_string(self) -> String;
    fn is_pressed(self) -> bool;
    fn is_pressed_repeat(self) -> bool;
    fn is_down(self) -> bool;
    fn is_released(self) -> bool;
    fn is_up(self) -> bool;
}

impl KeyboardKeyExt for KeyboardKey {
    fn get_pressed() -> Self {
        Rcore::__get_key_pressed()
    }

    fn get_pressed_char() -> String {
        Rcore::__get_char_pressed()
    }

    fn to_string(self) -> String {
        char::from_u32(self as u32)
            .map(|c| c.to_string())
            .unwrap_or("".to_owned())
    }

    fn is_pressed(self) -> bool {
        Rcore::__is_key_pressed(self)
    }

    fn is_pressed_repeat(self) -> bool {
        Rcore::__is_key_pressed_repeat(self)
    }

    fn is_down(self) -> bool {
        Rcore::__is_key_down(self)
    }

    fn is_released(self) -> bool {
        Rcore::__is_key_released(self)
    }

    fn is_up(self) -> bool {
        Rcore::__is_key_up(self)
    }
}
