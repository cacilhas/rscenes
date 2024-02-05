use crate::{assets::Codepoints, rtext::RtextImpl};
use raylib_ffi::*;
use std::fmt::Display;

pub trait FontExt: Sized {
    /// Get the default Font
    fn default() -> Self;
    /// Load font from file into GPU memory (VRAM)
    fn load(filename: impl Display) -> Result<Self, String>;
    /// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
    fn load_from_memory(tpe: impl Display, data: &[u8], font_size: i32) -> Result<Self, String>;
    /// Load font from Image (XNA style)
    fn load_from_image(image: Image, key: Color, first_char: i32) -> Result<Self, String>;
    /// Check whether a font is ready
    fn is_ready(self) -> bool;
    /// Unload font from GPU memory (VRAM)
    fn unload(self);
    /// Export font as code file, returns true on success
    fn export_as_code(self, filename: impl Display) -> bool;
}

impl FontExt for Font {
    fn default() -> Self {
        RtextImpl::__get_default_font()
    }

    fn load(filename: impl Display) -> Result<Self, String> {
        RtextImpl::__load_font(filename)
    }

    fn load_from_memory(tpe: impl Display, data: &[u8], font_size: i32) -> Result<Self, String> {
        RtextImpl::__load_font_from_memory(tpe, data, font_size, Codepoints::default())
    }

    fn load_from_image(image: Image, key: Color, first_char: i32) -> Result<Self, String> {
        RtextImpl::__load_font_from_image(image, key, first_char)
    }

    fn is_ready(self) -> bool {
        RtextImpl::__is_font_ready(self)
    }

    fn unload(self) {
        RtextImpl::__unload_font(self)
    }

    fn export_as_code(self, filename: impl Display) -> bool {
        RtextImpl::__export_font_as_code(self, filename)
    }
}
