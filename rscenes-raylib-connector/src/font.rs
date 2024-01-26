use std::fmt::Display;

use crate::rtext::Rtext;
use raylib_ffi::Font;

pub trait FontExt {
    fn is_ready(self) -> bool;
    fn unload(self);
    fn export_as_code(self, filename: impl Display) -> bool;
}

impl FontExt for Font {
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
