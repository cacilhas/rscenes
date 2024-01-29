use crate::rcore::RcoreImpl;
use raylib_ffi::enums::KeyboardKey;
use std::char;

pub trait KeyboardKeyExt: Sized {
    /// Get key pressed, call it multiple times for keys queued, returns KeyboardKey::Null when the queue is empty
    fn get_pressed() -> Self;
    /// Get char pressed (unicode), call it multiple times for chars queued, returns empty when the queue is empty
    fn get_pressed_char() -> String;
    /// Convert to string
    fn to_string(self) -> String;
    /// Check whether a key has been pressed once
    fn is_pressed(self) -> bool;
    /// Check whether a key has been pressed again (Only PLATFORM_DESKTOP)
    fn is_pressed_repeat(self) -> bool;
    /// Check whether a key is being pressed
    fn is_down(self) -> bool;
    /// Check whether a key has been released once
    fn is_released(self) -> bool;
    /// Check whether a key is NOT being pressed
    fn is_up(self) -> bool;
}

impl KeyboardKeyExt for KeyboardKey {
    fn get_pressed() -> Self {
        RcoreImpl::__get_key_pressed()
    }

    fn get_pressed_char() -> String {
        RcoreImpl::__get_char_pressed()
    }

    fn to_string(self) -> String {
        char::from_u32(self as u32)
            .map(|c| c.to_string())
            .unwrap_or("".to_owned())
    }

    fn is_pressed(self) -> bool {
        RcoreImpl::__is_key_pressed(self)
    }

    fn is_pressed_repeat(self) -> bool {
        RcoreImpl::__is_key_pressed_repeat(self)
    }

    fn is_down(self) -> bool {
        RcoreImpl::__is_key_down(self)
    }

    fn is_released(self) -> bool {
        RcoreImpl::__is_key_released(self)
    }

    fn is_up(self) -> bool {
        RcoreImpl::__is_key_up(self)
    }
}
