use crate::{
    ext::codepoints::Codepoints,
    utils::{string_from_c, utf8_from_c},
};
use raylib_ffi::*;
use std::{ffi::c_uchar, fmt::Display};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rtext;

/// Crate only methods
impl Rtext {
    // Font loading/unloading methods

    pub(crate) fn __get_default_font() -> Font {
        unsafe { GetFontDefault() }
    }

    pub(crate) fn __load_font(filename: impl Display) -> Result<Font, String> {
        unsafe {
            let font = LoadFont(rl_str!(filename));
            if font.baseSize > 0 {
                Ok(font)
            } else {
                Err(format!("couldn't load font from {}", filename))
            }
        }
    }

    pub(crate) fn __load_font_ex(
        filename: impl Display,
        font_size: i32,
        codepoints: Codepoints,
    ) -> Result<Font, String> {
        unsafe {
            let count = codepoints.count as i32;
            let font = LoadFontEx(rl_str!(filename), font_size, codepoints.into(), count);
            if font.baseSize > 0 {
                Ok(font)
            } else {
                Err(format!("couldn't load font from {}", filename))
            }
        }
    }

    pub(crate) fn __load_font_from_image(
        image: Image,
        key: Color,
        first_char: i32,
    ) -> Result<Font, String> {
        unsafe {
            let font = LoadFontFromImage(image, key, first_char);
            if font.baseSize > 0 {
                Ok(font)
            } else {
                Err("couldn't load font from image".to_owned())
            }
        }
    }

    pub(crate) fn __load_font_from_memory(
        tpe: impl Display,
        data: &[u8],
        font_size: i32,
        codepoints: Codepoints,
    ) -> Result<Font, String> {
        unsafe {
            let data_size = data.len() as i32;
            let codepoints_count = codepoints.len() as i32;
            let data = data.to_owned().as_mut_ptr() as *mut c_uchar;
            let font = LoadFontFromMemory(
                rl_str!(tpe),
                data,
                data_size,
                font_size,
                codepoints.into(),
                codepoints_count,
            );
            if font.baseSize > 0 {
                Ok(font)
            } else {
                Err("couldn't load font from memory".to_owned())
            }
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

    pub(crate) fn __load_utf8(codepoints: Codepoints) -> Result<String, String> {
        unsafe {
            let length = codepoints.len() as i32;
            let raw = LoadUTF8(codepoints.into(), length);
            let res = string_from_c(raw)?.clone();
            UnloadUTF8(raw);
            Ok(res)
        }
    }

    // TOD: UnloadUTF8

    pub(crate) fn __load_codepoints(text: impl Display) -> Result<Codepoints, String> {
        unsafe {
            let c_text = rl_str!(text);
            let mut count: i32 = 0;
            let ptr = LoadCodepoints(c_text, &mut count);
            let raw = Codepoints::new(ptr, count);
            if raw.inner.is_null() {
                Err(format!("couldn't load codepoints from: {}", text))
            } else {
                Ok(raw)
            }
        }
    }

    pub(crate) fn __unload_codepoints(codepoints: impl Into<*mut i32>) {
        unsafe { UnloadCodepoints(codepoints.into()) }
    }

    pub(crate) fn __get_codepoint_count(text: impl Display) -> i32 {
        unsafe { GetCodepointCount(rl_str!(text)) }
    }

    pub(crate) fn __get_codepoint(text: impl Display) -> Result<(i32, usize), String> {
        unsafe {
            let mut size: i32 = 0;
            let cp = GetCodepoint(rl_str!(text), &mut size);
            if cp == 0x3f {
                Err("error trying to get codepoint".to_owned())
            } else if size == 0 {
                Err("no codepoint found".to_owned())
            } else {
                Ok((cp, size as usize))
            }
        }
    }

    pub(crate) fn __get_codepoint_next(text: impl Display) -> Result<(i32, usize), String> {
        unsafe {
            let mut size: i32 = 0;
            let cp = GetCodepointNext(rl_str!(text), &mut size);
            if cp == 0x3f {
                Err("error trying to get codepoint".to_owned())
            } else if size == 0 {
                Err("no codepoint found".to_owned())
            } else {
                Ok((cp, size as usize))
            }
        }
    }

    pub(crate) fn __get_codepoint_previous(text: impl Display) -> Result<(i32, usize), String> {
        unsafe {
            let mut size: i32 = 0;
            let cp = GetCodepointPrevious(rl_str!(text), &mut size);
            if cp == 0x3f {
                Err("error trying to get codepoint".to_owned())
            } else if size == 0 {
                Err("no codepoint found".to_owned())
            } else {
                Ok((cp, size as usize))
            }
        }
    }

    pub(crate) fn __codepoint_to_utf8(codepoint: i32) -> Result<String, String> {
        unsafe {
            let mut size: i32 = 0;
            let raw = CodepointToUTF8(codepoint, &mut size) as *const u8;
            utf8_from_c(raw, size as usize)
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

    /// Get the default Font
    pub fn get_default_font(&self) -> Font {
        Self::__get_default_font()
    }

    /// Load font from file into GPU memory (VRAM)
    pub fn load_font(&self, filename: impl Display) -> Result<Font, String> {
        Self::__load_font(filename)
    }

    /// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set
    pub fn load_font_ex(
        &self,
        filename: impl Display,
        font_size: i32,
        codepoints: Codepoints,
    ) -> Result<Font, String> {
        Self::__load_font_ex(filename, font_size, codepoints)
    }

    /// Load font from Image (XNA style)
    pub fn load_font_from_image(
        &self,
        image: Image,
        key: Color,
        first_char: i32,
    ) -> Result<Font, String> {
        Self::__load_font_from_image(image, key, first_char)
    }

    /// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
    pub fn load_font_from_memory(
        &self,
        tpe: impl Display, // TODO: implement a font type enum
        data: &[u8],
        font_size: i32,
        codepoints: Codepoints,
    ) -> Result<Font, String> {
        Self::__load_font_from_memory(tpe, data, font_size, codepoints)
    }

    /// Check whether a font is ready
    pub fn is_font_ready(&self, font: Font) -> bool {
        Self::__is_font_ready(font)
    }

    /// Unload font from GPU memory (VRAM)
    pub fn unload_font(&self, font: Font) {
        Self::__unload_font(font)
    }

    /// Export font as code file, returns true on success
    pub fn export_font_as_code(&self, font: Font, filename: impl Display) -> bool {
        Self::__export_font_as_code(font, filename)
    }

    // Text drawing methods

    /// Draw current FPS
    pub fn draw_fps(&self, x: i32, y: i32) {
        Self::__draw_fps(x, y)
    }

    /// Draw text (using default font)
    pub fn draw_text(&self, text: impl Display, x: i32, y: i32, font_size: i32, color: Color) {
        Self::__draw_text(text, x, y, font_size, color)
    }

    /// Draw text using font and additional parameters
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

    /// Draw text using Font and pro parameters (rotation)
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

    /// Draw one character (codepoint)
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

    /// Draw multiple character (codepoint)
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

    /// Set vertical line spacing when drawing with line-breaks
    pub fn set_text_line_space(&self, spacing: i32) {
        Self::__set_text_line_space(spacing)
    }

    /// Measure string width for default font
    pub fn measure_text(&self, text: impl Display, font_size: i32) -> i32 {
        Self::__measure_text(text, font_size)
    }

    /// Measure string size for Font
    pub fn measure_text_ex(
        &self,
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
    ) -> Vector2 {
        Self::__measure_text_ex(font, text, font_size, spacing)
    }

    /// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
    pub fn get_glyph_index(&self, font: Font, codepoint: i32) -> i32 {
        Self::__get_glyph_index(font, codepoint)
    }

    /// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
    pub fn get_glyph_info(&self, font: Font, codepoint: i32) -> GlyphInfo {
        Self::__get_glyph_info(font, codepoint)
    }

    /// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
    pub fn get_glyph_atlas_rec(&self, font: Font, codepoint: i32) -> Rectangle {
        Self::__get_glyph_atlas_rec(font, codepoint)
    }

    // Text codepoints management methods (unicode characters)

    /// Load UTF-8 text encoded from codepoints array
    pub fn load_utf8(&self, codepoints: Codepoints) -> Result<String, String> {
        Self::__load_utf8(codepoints)
    }

    /// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
    pub fn load_codepoints(&self, text: impl Display) -> Result<Codepoints, String> {
        Self::__load_codepoints(text)
    }

    /// Unload codepoints data from memory
    pub fn unload_codepoints(&self, codepoints: Codepoints) {
        Self::__unload_codepoints(codepoints)
    }

    /// Get total number of codepoints in a UTF-8 encoded string
    pub fn get_codepoint_count(&self, text: impl Display) -> i32 {
        Self::__get_codepoint_count(text)
    }

    /// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
    pub fn get_codepoint(&self, text: impl Display) -> Result<(i32, usize), String> {
        Self::__get_codepoint(text)
    }

    /// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
    pub fn get_codepoint_next(&self, text: impl Display) -> Result<(i32, usize), String> {
        Self::__get_codepoint_next(text)
    }

    /// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
    pub fn get_codepoint_previous(&self, text: impl Display) -> Result<(i32, usize), String> {
        Self::__get_codepoint_previous(text)
    }

    /// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
    pub fn codepoint_to_utf8(&self, codepoint: i32) -> Result<String, String> {
        Self::__codepoint_to_utf8(codepoint)
    }
}
