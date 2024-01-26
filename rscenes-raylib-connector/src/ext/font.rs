use crate::rtext::Rtext;
use raylib_ffi::*;
use std::fmt::Display;

pub trait FontExt {
    fn default() -> Self;
    fn load(filename: impl Display) -> Self;
    fn load_from_image(image: Image, key: Color, first_char: i32) -> Self;
    fn is_ready(self) -> bool;
    fn unload(self);
    fn export_as_code(self, filename: impl Display) -> bool;
}

impl FontExt for Font {
    fn default() -> Self {
        Rtext::__get_default_font()
    }

    fn load(filename: impl Display) -> Self {
        Rtext::__load_font(filename)
    }

    fn load_from_image(image: Image, key: Color, first_char: i32) -> Self {
        Rtext::__load_font_from_image(image, key, first_char)
    }

    fn is_ready(self) -> bool {
        Rtext::__is_font_ready(self)
    }

    fn unload(self) {
        Rtext::__unload_font(self)
    }

    fn export_as_code(self, filename: impl Display) -> bool {
        Rtext::__export_font_as_code(self, filename)
    }
}
