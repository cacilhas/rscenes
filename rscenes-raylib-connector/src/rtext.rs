use crate::codepoints::Codepoints;
use eyre::*;
use raylib_ffi::*;
use std::{
    ffi::{c_uchar, CString},
    fmt::Display,
    slice,
};

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
        codepoints: Codepoints,
    ) -> Font {
        unsafe {
            let count = codepoints.count as i32;
            LoadFontEx(rl_str!(filename), font_size, codepoints.into(), count)
        }
    }

    pub(self) fn __load_font_from_image(image: Image, key: Color, first_char: i32) -> Font {
        unsafe { LoadFontFromImage(image, key, first_char) }
    }

    pub(crate) fn __load_font_from_memory(
        tpe: impl Display,
        data: Vec<u8>,
        font_size: i32,
        codepoints: Codepoints,
    ) -> Font {
        unsafe {
            let data_size = data.len() as i32;
            let codepoints_count = codepoints.len() as i32;
            let data = data.as_ptr() as *mut c_uchar;
            LoadFontFromMemory(
                rl_str!(tpe),
                data,
                data_size,
                font_size,
                codepoints.into(),
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
        codepoints: Codepoints,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        unsafe {
            let count = codepoints.len() as i32;
            DrawTextCodepoints(
                font,
                codepoints.into(),
                count,
                position,
                font_size,
                spacing,
                tint,
            )
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

    pub(crate) fn __load_utf8(codepoints: Codepoints) -> Result<String> {
        unsafe {
            let length = codepoints.len() as i32;
            Ok(CString::from_raw(LoadUTF8(codepoints.into(), length)).into_string()?)
        }
    }

    // TOD: UnloadUTF8

    pub(crate) fn __load_codepoints(text: impl Display) -> Codepoints {
        unsafe {
            let text = rl_str!(text);
            let mut count: i32 = 0;
            let ptr = LoadCodepoints(text, &mut count);
            Codepoints::new(ptr, count)
        }
    }

    pub(crate) fn __unload_codepoints(codepoints: impl Into<*mut i32>) {
        unsafe { UnloadCodepoints(codepoints.into()) }
    }

    pub(crate) fn __get_codepoint_count(text: impl Display) -> i32 {
        unsafe { GetCodepointCount(rl_str!(text)) }
    }

    pub(crate) fn __get_codepoint(text: impl Display) -> Result<(i32, usize)> {
        unsafe {
            let mut size: i32 = 0;
            let cp = GetCodepoint(rl_str!(text), &mut size);
            if cp == 0x3f {
                Err(eyre!("error trying to get codepoint"))
            } else if size == 0 {
                Err(eyre!("no codepoint found"))
            } else {
                Ok((cp, size as usize))
            }
        }
    }

    pub(crate) fn __get_codepoint_next(text: impl Display) -> Result<(i32, usize)> {
        unsafe {
            let mut size: i32 = 0;
            let cp = GetCodepointNext(rl_str!(text), &mut size);
            if cp == 0x3f {
                Err(eyre!("error trying to get codepoint"))
            } else if size == 0 {
                Err(eyre!("no codepoint found"))
            } else {
                Ok((cp, size as usize))
            }
        }
    }

    pub(crate) fn __get_codepoint_previous(text: impl Display) -> Result<(i32, usize)> {
        unsafe {
            let mut size: i32 = 0;
            let cp = GetCodepointPrevious(rl_str!(text), &mut size);
            if cp == 0x3f {
                Err(eyre!("error trying to get codepoint"))
            } else if size == 0 {
                Err(eyre!("no codepoint found"))
            } else {
                Ok((cp, size as usize))
            }
        }
    }

    pub(crate) fn __codepoint_to_utf8(codepoint: i32) -> Result<String> {
        unsafe {
            let mut size: i32 = 0;
            let res = CodepointToUTF8(codepoint, &mut size) as *const u8;
            let bytes = slice::from_raw_parts(res, size as usize);
            Ok(std::str::from_utf8(bytes).map(|s| s.to_owned())?)
        }
    }

    // Text strings management methods (no UTF-8 strings, only byte chars)
    //
    // The following aren’t required, ’cause Rust supplies those features by
    // itself:
    // int TextCopy(char *dst, const char *src);
    // bool TextIsEqual(const char *text1, const char *text2);
    // unsigned int TextLength(const char *text);
    // const char *TextFormat(const char *text, ...);
    // const char *TextSubtext(const char *text, int position, int length);
    // char *TextReplace(char *text, const char *replace, const char *by);
    // char *TextInsert(const char *text, const char *insert, int position);
    // const char *TextJoin(const char **textList, int count, const char *delimiter);
    // const char **TextSplit(const char *text, char delimiter, int *count);
    // void TextAppend(char *text, const char *append, int *position);
    // int TextFindIndex(const char *text, const char *find);
    // const char *TextToUpper(const char *text);
    // const char *TextToLower(const char *text);
    // const char *TextToPascal(const char *text);
    // int TextToInteger(const char *text);
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
        codepoints: Codepoints,
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
        codepoints: Codepoints,
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
        codepoints: Codepoints,
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

    pub fn load_utf8(&self, codepoints: Codepoints) -> Result<String> {
        Self::__load_utf8(codepoints)
    }

    pub fn load_codepoints(&self, text: impl Display) -> Codepoints {
        Self::__load_codepoints(text)
    }

    pub fn unload_codepoints(&self, codepoints: Codepoints) {
        Self::__unload_codepoints(codepoints)
    }

    pub fn get_codepoint_count(&self, text: impl Display) -> i32 {
        Self::__get_codepoint_count(text)
    }

    pub fn get_codepoint(&self, text: impl Display) -> Result<(i32, usize)> {
        Self::__get_codepoint(text)
    }

    pub fn get_codepoint_next(&self, text: impl Display) -> Result<(i32, usize)> {
        Self::__get_codepoint_next(text)
    }

    pub fn get_codepoint_previous(&self, text: impl Display) -> Result<(i32, usize)> {
        Self::__get_codepoint_previous(text)
    }

    pub fn codepoint_to_utf8(&self, codepoint: i32) -> Result<String> {
        Self::__codepoint_to_utf8(codepoint)
    }
}
