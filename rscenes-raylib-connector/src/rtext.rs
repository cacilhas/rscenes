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

    pub(crate) fn __draw_fps(x: i32, y: i32) {
        unsafe { DrawFPS(x, y) }
    }

    pub(crate) fn __draw_text(text: impl Display, x: i32, y: i32, font_size: i32, color: Color) {
        unsafe { DrawText(rl_str!(text), x, y, font_size, color) }
    }

    pub(crate) fn __draw_text_ex(
        font: Font,
        text: impl Display,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        unsafe { DrawTextEx(font, rl_str!(text), position, font_size, spacing, tint) }
    }

    pub(crate) fn __draw_text_pro(
        font: Font,
        text: impl Display,
        position: Vector2,
        origin: Vector2,
        rotation: f32,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        unsafe {
            DrawTextPro(
                font,
                rl_str!(text),
                position,
                origin,
                rotation,
                font_size,
                spacing,
                tint,
            )
        }
    }

    pub(crate) fn __draw_text_codepoint(
        font: Font,
        codepoint: i32,
        position: Vector2,
        font_size: f32,
        tint: Color,
    ) {
        unsafe { DrawTextCodepoint(font, codepoint, position, font_size, tint) }
    }

    pub(crate) fn __draw_text_codepoints(
        font: Font,
        codepoints: Vec<i32>,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        unsafe {
            let count = codepoints.len() as i32;
            let codepoints = codepoints.as_ptr() as *mut i32;
            DrawTextCodepoints(font, codepoints, count, position, font_size, spacing, tint)
        }
    }

    // Text font info methods

    pub(crate) fn __set_text_line_space(spacing: i32) {
        unsafe { SetTextLineSpacing(spacing) }
    }

    pub(crate) fn __measure_text(text: impl Display, font_size: i32) -> i32 {
        unsafe { MeasureText(rl_str!(text), font_size) }
    }

    pub(crate) fn __measure_text_ex(
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
    ) -> Vector2 {
        unsafe { MeasureTextEx(font, rl_str!(text), font_size, spacing) }
    }

    pub(crate) fn __get_glyph_index(font: Font, codepoint: i32) -> i32 {
        unsafe { GetGlyphIndex(font, codepoint) }
    }

    pub(crate) fn __get_glyph_info(font: Font, codepoint: i32) -> GlyphInfo {
        unsafe { GetGlyphInfo(font, codepoint) }
    }

    pub(crate) fn __get_glyph_atlas_rec(font: Font, codepoint: i32) -> Rectangle {
        unsafe { GetGlyphAtlasRec(font, codepoint) }
    }

    // Text codepoints management methods (unicode characters)
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

    pub fn draw_fps(&self, x: i32, y: i32) {
        Self::__draw_fps(x, y)
    }

    pub fn draw_text(&self, text: impl Display, x: i32, y: i32, font_size: i32, color: Color) {
        Self::__draw_text(text, x, y, font_size, color)
    }

    pub fn draw_text_ex(
        &self,
        font: Font,
        text: impl Display,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        Self::__draw_text_ex(font, text, position, font_size, spacing, tint)
    }

    pub fn draw_text_pro(
        &self,
        font: Font,
        text: impl Display,
        position: Vector2,
        origin: Vector2,
        rotation: f32,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        Self::__draw_text_pro(
            font, text, position, origin, rotation, font_size, spacing, tint,
        )
    }

    pub fn draw_text_codepoint(
        &self,
        font: Font,
        codepoint: i32,
        position: Vector2,
        font_size: f32,
        tint: Color,
    ) {
        Self::__draw_text_codepoint(font, codepoint, position, font_size, tint)
    }

    pub fn draw_text_codepoints(
        &self,
        font: Font,
        codepoints: Vec<i32>,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        Self::__draw_text_codepoints(font, codepoints, position, font_size, spacing, tint)
    }

    // Text font info methods

    pub fn set_text_line_space(&self, spacing: i32) {
        Self::__set_text_line_space(spacing)
    }

    pub fn measure_text(&self, text: impl Display, font_size: i32) -> i32 {
        Self::__measure_text(text, font_size)
    }

    pub fn measure_text_ex(
        &self,
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
    ) -> Vector2 {
        Self::__measure_text_ex(font, text, font_size, spacing)
    }

    pub fn get_glyph_index(&self, font: Font, codepoint: i32) -> i32 {
        Self::__get_glyph_index(font, codepoint)
    }

    pub fn get_glyph_info(&self, font: Font, codepoint: i32) -> GlyphInfo {
        Self::__get_glyph_info(font, codepoint)
    }

    pub fn get_glyph_atlas_rec(&self, font: Font, codepoint: i32) -> Rectangle {
        Self::__get_glyph_atlas_rec(font, codepoint)
    }

    // Text codepoints management methods (unicode characters)
}
