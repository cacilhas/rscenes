use std::{ffi::c_uchar, fmt::Display};

use raylib_ffi::*;

#[derive(Clone, Copy, Debug)]
pub struct Rtext;

/// Crate only methods
impl Rtext {
    // Font loading/unloading methods

    pub(crate) fn __get_default_font() -> Font {
        unsafe { GetFontDefault() }
    }

    pub(crate) fn __load_font(filename: impl Display) -> Font {
        unsafe { LoadFont(rl_str!(filename)) }
    }

    pub(crate) fn __load_font_ex(
        filename: impl Display,
        font_size: i32,
        codepoints: Vec<i32>,
    ) -> Font {
        unsafe {
            let count = codepoints.len() as i32;
            let codepoints = codepoints.as_ptr() as *mut i32;
            LoadFontEx(rl_str!(filename), font_size, codepoints, count)
        }
    }

    pub(self) fn __load_font_from_image(image: Image, key: Color, first_char: i32) -> Font {
        unsafe { LoadFontFromImage(image, key, first_char) }
    }

    pub(crate) fn __load_font_from_memory(
        tpe: impl Display,
        data: Vec<u8>,
        font_size: i32,
        codepoints: Vec<i32>,
    ) -> Font {
        unsafe {
            let data_size = data.len() as i32;
            let codepoints_count = codepoints.len() as i32;
            let data = data.as_ptr() as *mut c_uchar;
            let codepoints = codepoints.as_ptr() as *mut i32;
            LoadFontFromMemory(
                rl_str!(tpe),
                data,
                data_size,
                font_size,
                codepoints,
                codepoints_count,
            )
        }
    }

    pub(crate) fn __is_font_ready(font: Font) -> bool {
        unsafe { IsFontReady(font) }
    }

    // TODO: LoadFontData
    // TODO: GenImageFontAtlas
    // TODO: UnloadFontData

    pub(crate) fn __unload_font(font: Font) {
        unsafe { UnloadFont(font) }
    }

    pub(crate) fn __export_font_as_code(font: Font, filename: impl Display) -> bool {
        unsafe { ExportFontAsCode(font, rl_str!(filename)) }
    }

    // Text drawing methods
}

/// Exported methods
impl Rtext {
    // Font loading/unloading methods

    pub fn get_default_font(&self) -> Font {
        Self::__get_default_font()
    }

    pub fn load_font(&self, filename: impl Display) -> Font {
        Self::__load_font(filename)
    }

    pub fn load_font_ex(
        &self,
        filename: impl Display,
        font_size: i32,
        codepoints: Vec<i32>,
    ) -> Font {
        Self::__load_font_ex(filename, font_size, codepoints)
    }

    pub fn load_font_from_image(&self, image: Image, key: Color, first_char: i32) -> Font {
        Self::__load_font_from_image(image, key, first_char)
    }

    pub fn load_font_from_memory(
        &self,
        tpe: impl Display,
        data: Vec<u8>,
        font_size: i32,
        codepoints: Vec<i32>,
    ) -> Font {
        Self::__load_font_from_memory(tpe, data, font_size, codepoints)
    }

    pub fn is_font_ready(&self, font: Font) -> bool {
        Self::__is_font_ready(font)
    }

    pub fn unload_font(&self, font: Font) {
        Self::__unload_font(font)
    }

    pub fn export_font_as_code(&self, font: Font, filename: impl Display) -> bool {
        Self::__export_font_as_code(font, filename)
    }

    // Text drawing methods
}
