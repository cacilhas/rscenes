use crate::{ext::image::ImageType, utils::array_from_c};
use raylib_ffi::{enums::*, *};
use std::{
    f32::consts::PI,
    ffi::c_void,
    fmt::{Debug, Display},
    path::Path,
};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RtexturesImpl;

/// Crate only methods
impl RtexturesImpl {
    // Image loading

    pub fn __load_image(filename: impl Display) -> Result<Image, String> {
        unsafe {
            let image = LoadImage(rl_str!(filename));
            if image.data.is_null() {
                if Path::new(&format!("{}", filename)).exists() {
                    Err(format!("couldn't load image from {}", filename))
                } else {
                    Err(format!(
                        "couldn't load image from {}, file not found",
                        filename
                    ))
                }
            } else {
                Ok(image)
            }
        }
    }

    pub fn __load_image_raw(
        filename: impl Display,
        width: i32,
        height: i32,
        format: impl Into<usize>,
        header_size: i32,
    ) -> Result<Image, String> {
        unsafe {
            let image = LoadImageRaw(
                rl_str!(filename),
                width,
                height,
                format.into() as i32,
                header_size,
            );
            if image.data.is_null() {
                if Path::new(&format!("{}", filename)).exists() {
                    Err(format!("couldn't load image from {}", filename))
                } else {
                    Err(format!(
                        "couldn't load image from {}, file not found",
                        filename
                    ))
                }
            } else {
                Ok(image)
            }
        }
    }

    pub fn __load_image_svg(
        filename_or_string: impl Display,
        width: i32,
        height: i32,
    ) -> Result<Image, String> {
        unsafe {
            let image = LoadImageSvg(rl_str!(filename_or_string), width, height);
            if image.data.is_null() {
                Err(format!("couldn't load image from {}", filename_or_string))
            } else {
                Ok(image)
            }
        }
    }

    pub fn __load_image_anim(filename: impl Display) -> Result<(Image, i32), String> {
        unsafe {
            let mut frames: i32 = 0;
            let image = LoadImageAnim(rl_str!(filename), &mut frames);
            if image.data.is_null() {
                if Path::new(&format!("{}", filename)).exists() {
                    Err(format!("couldn't load image from {}", filename))
                } else {
                    Err(format!(
                        "couldn't load image from {}, file not found",
                        filename
                    ))
                }
            } else {
                Ok((image, frames))
            }
        }
    }

    // FIXME: failing to load PNG
    pub fn __load_image_from_memory(tpe: impl Display, data: &[u8]) -> Result<Image, String> {
        unsafe {
            let size = data.len() as i32;
            let mut data = data.iter().map(|e| *e).collect::<Vec<_>>();
            let data = data.as_mut_ptr();
            let image = LoadImageFromMemory(rl_str!(tpe), data, size);
            if image.data.is_null() {
                Err("failed to load image from memory".to_owned())
            } else {
                Ok(image)
            }
        }
    }

    pub fn __load_image_from_texture(texture: Texture2D) -> Image {
        unsafe { LoadImageFromTexture(texture) }
    }

    pub fn __load_image_from_screen() -> Image {
        unsafe { LoadImageFromScreen() }
    }

    pub fn __is_image_ready(image: Image) -> bool {
        unsafe { IsImageReady(image) }
    }

    pub fn __unload_image(image: Image) {
        unsafe { UnloadImage(image) }
    }

    pub fn __export_image(image: Image, filename: impl Display) -> bool {
        unsafe { ExportImage(image, rl_str!(filename)) }
    }

    pub fn __export_image_to_memory(image: Image, tpe: impl Display) -> Result<Vec<u8>, String> {
        unsafe {
            let mut size: i32 = 0;
            let raw = ExportImageToMemory(image, rl_str!(tpe), &mut size);
            array_from_c(raw, size as usize, || "failed to export image".to_owned())
        }
    }

    pub fn __export_image_as_code(image: Image, filename: impl Display) -> bool {
        unsafe { ExportImageAsCode(image, rl_str!(filename)) }
    }

    // Image generation methods

    pub fn __gen_image_color(width: i32, height: i32, color: Color) -> Image {
        unsafe { GenImageColor(width, height, color) }
    }

    pub fn __gen_image_gradient_linear(
        width: i32,
        height: i32,
        angle: f32,
        start: Color,
        end: Color,
    ) -> Image {
        unsafe {
            let direction = (angle * 180.0 / PI) as i32;
            GenImageGradientLinear(width, height, direction, start, end)
        }
    }

    pub fn __gen_image_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        unsafe { GenImageGradientRadial(width, height, density, inner, outer) }
    }

    pub fn __gen_image_gradient_square(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        unsafe { GenImageGradientSquare(width, height, density, inner, outer) }
    }

    pub fn __gen_image_checked(
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: Color,
        col2: Color,
    ) -> Image {
        unsafe { GenImageChecked(width, height, checks_x, checks_y, col1, col2) }
    }

    pub fn __gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
        unsafe { GenImageWhiteNoise(width, height, factor) }
    }

    pub fn __gen_image_perlin_noise(
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Image {
        unsafe { GenImagePerlinNoise(width, height, offset_x, offset_y, scale) }
    }

    pub fn __gen_image_cellular(width: i32, height: i32, tile_size: i32) -> Image {
        unsafe { GenImageCellular(width, height, tile_size) }
    }

    pub fn __gen_image_text(width: i32, height: i32, text: impl Display) -> Image {
        unsafe { GenImageText(width, height, rl_str!(text)) }
    }

    // Image manipulation methods

    pub fn __image_copy(image: Image) -> Image {
        unsafe { ImageCopy(image) }
    }

    pub fn __image_from_image(image: Image, rec: Rectangle) -> Image {
        unsafe { ImageFromImage(image, rec) }
    }

    pub fn __image_text(text: impl Display, font_size: i32, color: Color) -> Image {
        unsafe { ImageText(rl_str!(text), font_size, color) }
    }

    pub fn __image_text_ex(
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Image {
        unsafe { ImageTextEx(font, rl_str!(text), font_size, spacing, tint) }
    }

    pub fn __image_format(image: &mut Image, format: impl Into<usize>) {
        unsafe { ImageFormat(image, format.into() as i32) }
    }

    pub fn __image_to_pot(image: &mut Image, fill: Color) {
        unsafe { ImageToPOT(image, fill) }
    }

    pub fn __image_crop(image: &mut Image, crop: Rectangle) {
        unsafe { ImageCrop(image, crop) }
    }

    pub fn __image_alpha_crop(image: &mut Image, threshold: f32) {
        unsafe { ImageAlphaCrop(image, threshold) }
    }

    pub fn __image_alpha_clear(image: &mut Image, color: Color, threshold: f32) {
        unsafe { ImageAlphaClear(image, color, threshold) }
    }

    pub fn __image_alpha_mask(image: &mut Image, alpha_mask: Image) {
        unsafe { ImageAlphaMask(image, alpha_mask) }
    }

    pub fn __image_alpha_premultiply(image: &mut Image) {
        unsafe { ImageAlphaPremultiply(image) }
    }

    pub fn __image_blur_gaussian(image: &mut Image, blur_size: i32) {
        unsafe { ImageBlurGaussian(image, blur_size) }
    }

    pub fn __image_resize(image: &mut Image, width: i32, height: i32) {
        unsafe { ImageResize(image, width, height) }
    }

    pub fn __image_resize_nn(image: &mut Image, width: i32, height: i32) {
        unsafe { ImageResizeNN(image, width, height) }
    }

    pub fn __image_resize_canvas(
        image: &mut Image,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) {
        unsafe { ImageResizeCanvas(image, width, height, offset_x, offset_y, fill) }
    }

    pub fn __image_mipmaps(image: &mut Image) {
        unsafe { ImageMipmaps(image) }
    }

    pub fn __image_dither(image: &mut Image, r: i32, g: i32, b: i32, a: i32) {
        unsafe { ImageDither(image, r, g, b, a) }
    }

    pub fn __image_flip_vertical(image: &mut Image) {
        unsafe { ImageFlipVertical(image) }
    }

    pub fn __image_flip_horizontal(image: &mut Image) {
        unsafe { ImageFlipHorizontal(image) }
    }

    pub fn __image_rotate(image: &mut Image, angle: f32) {
        unsafe {
            let degrees = (angle * 180.0 / PI) as i32;
            ImageRotate(image, degrees)
        }
    }

    pub fn __image_rotate_cw(image: &mut Image) {
        unsafe { ImageRotateCW(image) }
    }

    pub fn __image_rotate_ccw(image: &mut Image) {
        unsafe { ImageRotateCCW(image) }
    }

    pub fn __image_color_tint(image: &mut Image, tint: Color) {
        unsafe { ImageColorTint(image, tint) }
    }

    pub fn __image_color_invert(image: &mut Image) {
        unsafe { ImageColorInvert(image) }
    }

    pub fn __image_color_grayscale(image: &mut Image) {
        unsafe { ImageColorGrayscale(image) }
    }

    pub fn __image_color_contrast(image: &mut Image, contrast: f32) {
        unsafe { ImageColorContrast(image, contrast) }
    }

    pub fn __image_color_brightness(image: &mut Image, brightness: i32) {
        unsafe { ImageColorBrightness(image, brightness) }
    }

    pub fn __image_color_replace(image: &mut Image, color: Color, replace: Color) {
        unsafe { ImageColorReplace(image, color, replace) }
    }

    pub fn __load_image_colors(image: Image) -> *mut Color {
        unsafe { LoadImageColors(image) }
    }

    pub fn __load_image_pallete(image: Image, max_size: i32) -> Result<Vec<Color>, String> {
        unsafe {
            let mut size: i32 = 0;
            let raw = LoadImagePalette(image, max_size, &mut size);
            let res = array_from_c(raw, size as usize, || {
                "failed to load pallet from image".to_owned()
            })?
            // Copy array elements to the stack
            .iter()
            .map(|e| Color {
                r: e.r,
                g: e.g,
                b: e.b,
                a: e.a,
            })
            .collect::<Vec<_>>();
            UnloadImagePalette(raw);
            Ok(res)
        }
    }

    pub fn __unload_image_colors(colors: *mut Color) {
        unsafe { UnloadImageColors(colors) }
    }

    pub fn __get_image_alpha_border(image: Image, threshold: f32) -> Rectangle {
        unsafe { GetImageAlphaBorder(image, threshold) }
    }

    pub fn __get_image_color(image: Image, x: i32, y: i32) -> Color {
        unsafe { GetImageColor(image, x, y) }
    }

    // Image drawing methods

    pub fn __image_clear_background(image: &mut Image, color: Color) {
        unsafe { ImageClearBackground(image, color) }
    }

    pub fn __image_draw_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
        unsafe { ImageDrawPixel(image, x, y, color) }
    }

    pub fn __image_draw_pixel_v(image: &mut Image, position: Vector2, color: Color) {
        unsafe { ImageDrawPixelV(image, position, color) }
    }

    pub fn __image_draw_line(
        image: &mut Image,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Color,
    ) {
        unsafe { ImageDrawLine(image, start_x, start_y, end_x, end_y, color) }
    }

    pub fn __image_draw_line_v(image: &mut Image, start: Vector2, end: Vector2, color: Color) {
        unsafe { ImageDrawLineV(image, start, end, color) }
    }

    pub fn __image_draw_circle(
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        unsafe { ImageDrawCircle(image, center_x, center_y, radius, color) }
    }

    pub fn __image_draw_circle_v(image: &mut Image, center: Vector2, radius: i32, color: Color) {
        unsafe { ImageDrawCircleV(image, center, radius, color) }
    }

    pub fn __image_draw_circle_lines(
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        unsafe { ImageDrawCircleLines(image, center_x, center_y, radius, color) }
    }

    pub fn __image_draw_circle_lines_v(
        image: &mut Image,
        center: Vector2,
        radius: i32,
        color: Color,
    ) {
        unsafe { ImageDrawCircleLinesV(image, center, radius, color) }
    }

    pub fn __image_draw_rectangle(
        image: &mut Image,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) {
        unsafe { ImageDrawRectangle(image, x, y, width, height, color) }
    }

    pub fn __image_draw_rectangle_v(
        image: &mut Image,
        position: Vector2,
        size: Vector2,
        color: Color,
    ) {
        unsafe { ImageDrawRectangleV(image, position, size, color) }
    }

    pub fn __image_draw_rectangle_rec(image: &mut Image, rec: Rectangle, color: Color) {
        unsafe { ImageDrawRectangleRec(image, rec, color) }
    }

    pub fn __image_draw_rectangle_lines(
        image: &mut Image,
        rec: Rectangle,
        thick: i32,
        color: Color,
    ) {
        unsafe { ImageDrawRectangleLines(image, rec, thick, color) }
    }

    pub fn __image_draw(
        image: &mut Image,
        src: Image,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: Color,
    ) {
        unsafe { ImageDraw(image, src, src_rec, dst_rec, tint) }
    }

    pub fn __image_draw_text(
        image: &mut Image,
        text: impl Display,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) {
        unsafe { ImageDrawText(image, rl_str!(text), x, y, font_size, color) }
    }

    pub fn __image_draw_text_ex(
        image: &mut Image,
        font: Font,
        text: impl Display,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        unsafe {
            ImageDrawTextEx(
                image,
                font,
                rl_str!(text),
                position,
                font_size,
                spacing,
                tint,
            )
        }
    }

    // Texture loading methods

    pub fn __load_texture(filename: impl Display) -> Result<Texture2D, String> {
        unsafe {
            let texture = LoadTexture(rl_str!(filename));
            if texture.id == 0 {
                if Path::new(&format!("{}", filename)).exists() {
                    Err(format!("couldn't load texture from {}", filename))
                } else {
                    Err(format!(
                        "couldn't load texture from {}, file not found",
                        filename
                    ))
                }
            } else {
                Ok(texture)
            }
        }
    }

    pub fn __load_texture_from_image(image: Image) -> Result<Texture2D, String> {
        unsafe {
            let texture = LoadTextureFromImage(image);
            if texture.id == 0 {
                Err("failed to load texture from image".to_owned())
            } else {
                Ok(texture)
            }
        }
    }

    pub fn __load_texture_cubemap(
        image: Image,
        layout: impl Into<usize>,
    ) -> Result<TextureCubemap, String> {
        unsafe {
            let texture = LoadTextureCubemap(image, layout.into() as i32);
            if texture.id == 0 {
                Err("failed to load cubemap from image".to_owned())
            } else {
                Ok(texture)
            }
        }
    }

    pub fn __load_render_texture(width: i32, height: i32) -> RenderTexture2D {
        unsafe { LoadRenderTexture(width, height) }
    }

    pub fn __is_texture_ready(texture: Texture2D) -> bool {
        unsafe { IsTextureReady(texture) }
    }

    pub fn __unload_texture(texture: Texture2D) {
        unsafe { UnloadTexture(texture) }
    }

    pub fn __is_render_texture_ready(target: RenderTexture2D) -> bool {
        unsafe { IsRenderTextureReady(target) }
    }

    pub fn __unload_render_texture(target: RenderTexture2D) {
        unsafe { UnloadRenderTexture(target) }
    }

    pub fn __update_texture(texture: Texture2D, pixels: &[u8]) {
        unsafe { UpdateTexture(texture, pixels.as_ptr() as *const c_void) }
    }

    pub fn __update_texture_rec(texture: Texture2D, rec: Rectangle, pixels: &[u8]) {
        unsafe { UpdateTextureRec(texture, rec, pixels.as_ptr() as *const c_void) }
    }

    // Texture configuration methods

    pub fn __gen_texture_mipmaps(texture: &mut Texture2D) {
        unsafe { GenTextureMipmaps(texture) }
    }

    pub fn __set_texture_filter(texture: Texture2D, filter: impl Into<usize>) {
        unsafe { SetTextureFilter(texture, filter.into() as i32) }
    }

    pub fn __set_texture_wrap(texture: Texture2D, wrap: impl Into<usize>) {
        unsafe { SetTextureWrap(texture, wrap.into() as i32) }
    }

    // Texture drawing methods

    pub fn __draw_texture(texture: Texture2D, x: i32, y: i32, tint: Color) {
        unsafe { DrawTexture(texture, x, y, tint) }
    }

    pub fn __draw_texture_v(texture: Texture2D, position: Vector2, tint: Color) {
        unsafe { DrawTextureV(texture, position, tint) }
    }

    pub fn __draw_texture_ex(
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        unsafe { DrawTextureEx(texture, position, rotation, scale, tint) }
    }

    pub fn __draw_texture_rec(
        texture: Texture2D,
        source: Rectangle,
        position: Vector2,
        tint: Color,
    ) {
        unsafe { DrawTextureRec(texture, source, position, tint) }
    }

    pub fn __draw_texture_pro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe { DrawTexturePro(texture, source, dest, origin, rotation, tint) }
    }

    pub fn __draw_texture_n_patch(
        texture: Texture2D,
        info: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe { DrawTextureNPatch(texture, info, dest, origin, rotation, tint) }
    }

    // Color/pixel related met

    pub fn __fade(color: Color, alpha: f32) -> Color {
        unsafe { Fade(color, alpha) }
    }

    pub fn __color_to_int(color: Color) -> i32 {
        unsafe { ColorToInt(color) }
    }

    pub fn __color_normalize(color: Color) -> Vector4 {
        unsafe { ColorNormalize(color) }
    }

    pub fn __color_from_normalized(normalized: Vector4) -> Color {
        unsafe { ColorFromNormalized(normalized) }
    }

    pub fn __color_to_hsv(color: Color) -> Vector3 {
        unsafe { ColorToHSV(color) }
    }

    pub fn __color_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        unsafe { ColorFromHSV(hue, saturation, value) }
    }

    pub fn __color_tint(color: Color, tint: Color) -> Color {
        unsafe { ColorTint(color, tint) }
    }

    pub fn __color_brightness(color: Color, factor: f32) -> Color {
        unsafe { ColorBrightness(color, factor) }
    }

    pub fn __color_contrast(color: Color, contrast: f32) -> Color {
        unsafe { ColorContrast(color, contrast) }
    }

    pub fn __color_alpha(color: Color, alpha: f32) -> Color {
        unsafe { ColorAlpha(color, alpha) }
    }

    pub fn __color_alpha_blend(dst: Color, src: Color, tint: Color) -> Color {
        unsafe { ColorAlphaBlend(dst, src, tint) }
    }

    pub fn __get_color(hex_value: u32) -> Color {
        unsafe { GetColor(hex_value) }
    }

    pub fn __get_pixel_color(ptr: &mut Vec<u8>, format: impl Into<usize>) -> Color {
        unsafe { GetPixelColor(ptr.as_mut_ptr() as *mut c_void, format.into() as i32) }
    }

    pub fn __set_pixel_color(ptr: &mut Vec<u8>, color: Color, format: impl Into<usize>) {
        unsafe { SetPixelColor(ptr.as_mut_ptr() as *mut c_void, color, format.into() as i32) }
    }

    pub fn __get_pixel_data_size(width: i32, height: i32, format: impl Into<usize>) -> i32 {
        unsafe { GetPixelDataSize(width, height, format.into() as i32) }
    }
}

/// Exported methods
pub trait Rtextures: Debug {
    // Image loading

    /// Load image from file into CPU memory (RAM)
    fn load_image(&self, filename: impl Display) -> Result<Image, String> {
        RtexturesImpl::__load_image(filename)
    }

    /// Load image from RAW file data
    fn load_image_raw(
        &self,
        filename: impl Display,
        width: i32,
        height: i32,
        format: PixelFormat,
        header_size: i32,
    ) -> Result<Image, String> {
        RtexturesImpl::__load_image_raw(filename, width, height, format, header_size)
    }

    /// Load image from SVG file data or string with specified size
    fn load_image_svg(
        &self,
        filename_or_string: impl Display,
        width: i32,
        height: i32,
    ) -> Result<Image, String> {
        RtexturesImpl::__load_image_svg(filename_or_string, width, height)
    }

    /// Load image sequence from file (frames appended to image.data)
    fn load_image_anim(&self, filename: impl Display) -> Result<(Image, i32), String> {
        RtexturesImpl::__load_image_anim(filename)
    }

    /// Load image from memory buffer, fileType refers to extension: i.e. '.png'
    fn load_image_from_memory(&self, tpe: ImageType, data: &[u8]) -> Result<Image, String> {
        RtexturesImpl::__load_image_from_memory(tpe, data)
    }

    /// Load image from GPU texture data
    fn load_image_from_texture(&self, texture: Texture2D) -> Image {
        RtexturesImpl::__load_image_from_texture(texture)
    }

    /// Load image from screen buffer and (screenshot)
    fn load_image_from_screen(&self) -> Image {
        RtexturesImpl::__load_image_from_screen()
    }

    /// Check whether an image is ready
    fn is_image_ready(&self, image: Image) -> bool {
        RtexturesImpl::__is_image_ready(image)
    }

    /// Unload image from CPU memory (RAM)
    fn unload_image(&self, image: Image) {
        RtexturesImpl::__unload_image(image)
    }

    /// Export image data to file, returns true on success
    fn export_image(&self, image: Image, filename: impl Display) -> bool {
        RtexturesImpl::__export_image(image, filename)
    }

    /// Export image to memory buffer
    fn export_image_to_memory(&self, image: Image, tpe: ImageType) -> Result<Vec<u8>, String> {
        RtexturesImpl::__export_image_to_memory(image, tpe)
    }

    /// Export image as code file defining an array of bytes, returns true on success
    fn export_image_as_code(&self, image: Image, filename: impl Display) -> bool {
        RtexturesImpl::__export_image_as_code(image, filename)
    }

    // Image generation methods

    /// Generate image: plain color
    fn gen_image_color(&self, width: i32, height: i32, color: Color) -> Image {
        RtexturesImpl::__gen_image_color(width, height, color)
    }

    /// Generate image: linear gradient, direction in radians, 0=Vertical gradient
    fn gen_image_gradient_linear(
        &self,
        width: i32,
        height: i32,
        angle: f32,
        start: Color,
        end: Color,
    ) -> Image {
        RtexturesImpl::__gen_image_gradient_linear(width, height, angle, start, end)
    }

    /// Generate image: radial gradient
    fn gen_image_gradient_radial(
        &self,
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        RtexturesImpl::__gen_image_gradient_radial(width, height, density, inner, outer)
    }

    // Generate image: square gradient
    fn gen_image_gradient_square(
        &self,
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        RtexturesImpl::__gen_image_gradient_square(width, height, density, inner, outer)
    }

    /// Generate image: checked
    fn gen_image_checked(
        &self,
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: Color,
        col2: Color,
    ) -> Image {
        RtexturesImpl::__gen_image_checked(width, height, checks_x, checks_y, col1, col2)
    }

    /// Generate image: white noise
    fn gen_image_white_noise(&self, width: i32, height: i32, factor: f32) -> Image {
        RtexturesImpl::__gen_image_white_noise(width, height, factor)
    }

    /// Generate image: perlin noise
    fn gen_image_perlin_noise(
        &self,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Image {
        RtexturesImpl::__gen_image_perlin_noise(width, height, offset_x, offset_y, scale)
    }

    /// Generate image: cellular algorithm, bigger tileSize means bigger cells
    fn gen_image_cellular(&self, width: i32, height: i32, tile_size: i32) -> Image {
        RtexturesImpl::__gen_image_cellular(width, height, tile_size)
    }

    /// Generate image: grayscale image from text data
    fn gen_image_text(&self, width: i32, height: i32, text: impl Display) -> Image {
        RtexturesImpl::__gen_image_text(width, height, text)
    }

    // Image manipulation methods

    /// Create an image duplicate (useful for transformations)
    fn image_copy(&self, image: Image) -> Image {
        RtexturesImpl::__image_copy(image)
    }

    /// Create an image from another image piece
    fn image_from_image(&self, image: Image, rec: Rectangle) -> Image {
        RtexturesImpl::__image_from_image(image, rec)
    }

    /// Create an image from text (default font)
    fn image_text(&self, text: impl Display, font_size: i32, color: Color) -> Image {
        RtexturesImpl::__image_text(text, font_size, color)
    }

    /// Create an image from text (custom sprite font)
    fn image_text_ex(
        &self,
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Image {
        RtexturesImpl::__image_text_ex(font, text, font_size, spacing, tint)
    }

    /// Convert image data to desired format
    fn image_format(&self, image: &mut Image, format: PixelFormat) {
        RtexturesImpl::__image_format(image, format)
    }

    /// Convert image to POT (power-of-two)
    fn image_to_pot(&self, image: &mut Image, fill: Color) {
        RtexturesImpl::__image_to_pot(image, fill)
    }

    /// Crop an image to a defined rectangle
    fn image_crop(&self, image: &mut Image, crop: Rectangle) {
        RtexturesImpl::__image_crop(image, crop)
    }

    /// Crop image depending on alpha value
    fn image_alpha_crop(&self, image: &mut Image, threshold: f32) {
        RtexturesImpl::__image_alpha_crop(image, threshold)
    }

    /// Clear alpha channel to desired color
    fn image_alpha_clear(&self, image: &mut Image, color: Color, threshold: f32) {
        RtexturesImpl::__image_alpha_clear(image, color, threshold)
    }

    /// Apply alpha mask to image
    fn image_alpha_mask(&self, image: &mut Image, alpha_mask: Image) {
        RtexturesImpl::__image_alpha_mask(image, alpha_mask)
    }

    /// Premultiply alpha channel
    fn image_alpha_premultiply(&self, image: &mut Image) {
        RtexturesImpl::__image_alpha_premultiply(image)
    }

    /// Apply Gaussian blur using a box blur approximation
    fn image_blur_gaussian(&self, image: &mut Image, blur_size: i32) {
        RtexturesImpl::__image_blur_gaussian(image, blur_size)
    }

    /// Resize image (Bicubic scaling algorithm)
    fn image_resize(&self, image: &mut Image, width: i32, height: i32) {
        RtexturesImpl::__image_resize(image, width, height)
    }

    /// Resize image (Nearest-Neighbor scaling algorithm)
    fn image_resize_nn(&self, image: &mut Image, width: i32, height: i32) {
        RtexturesImpl::__image_resize_nn(image, width, height)
    }

    // Resize canvas and fill with color
    fn image_resize_canvas(
        &self,
        image: &mut Image,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) {
        RtexturesImpl::__image_resize_canvas(image, width, height, offset_x, offset_y, fill)
    }

    /// Compute all mipmap levels for a provided image
    fn image_mipmaps(&self, image: &mut Image) {
        RtexturesImpl::__image_mipmaps(image)
    }

    /// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
    fn image_dither(&self, image: &mut Image, r: i32, g: i32, b: i32, a: i32) {
        RtexturesImpl::__image_dither(image, r, g, b, a)
    }

    /// Flip image vertically
    fn image_flip_vertical(&self, image: &mut Image) {
        RtexturesImpl::__image_flip_vertical(image)
    }

    /// Flip image horizontally
    fn image_flip_horizontal(&self, image: &mut Image) {
        RtexturesImpl::__image_flip_horizontal(image)
    }

    // Rotate image by input angle in radians
    fn image_rotate(&self, image: &mut Image, angle: f32) {
        RtexturesImpl::__image_rotate(image, angle)
    }

    /// Rotate image clockwise 90°
    fn image_rotate_cw(&self, image: &mut Image) {
        RtexturesImpl::__image_rotate_cw(image)
    }

    // Rotate image counter-clockwise 90°
    fn image_rotate_ccw(&self, image: &mut Image) {
        RtexturesImpl::__image_rotate_ccw(image)
    }

    /// Modify image color: tint
    fn image_color_tint(&self, image: &mut Image, tint: Color) {
        RtexturesImpl::__image_color_tint(image, tint)
    }

    /// Modify image color: invert
    fn image_color_invert(&self, image: &mut Image) {
        RtexturesImpl::__image_color_invert(image)
    }

    /// Modify image color: grayscale
    fn image_color_grayscale(&self, image: &mut Image) {
        RtexturesImpl::__image_color_grayscale(image)
    }

    /// Modify image color: contrast (-100 to 100)
    fn image_color_contrast(&self, image: &mut Image, contrast: f32) {
        RtexturesImpl::__image_color_contrast(image, contrast)
    }

    /// Modify image color: brightness (-255 to 255)
    fn image_color_brightness(&self, image: &mut Image, brightness: i32) {
        RtexturesImpl::__image_color_brightness(image, brightness)
    }

    /// Modify image color: replace color
    fn image_color_replace(&self, image: &mut Image, color: Color, replace: Color) {
        RtexturesImpl::__image_color_replace(image, color, replace)
    }

    /// Load color data from image as a Color array (RGBA - 32bit)
    fn load_image_colors(&self, image: Image) -> *mut Color {
        RtexturesImpl::__load_image_colors(image)
    }

    /// Load colors palette from image as a Color array (RGBA - 32bit)
    fn load_image_pallete(&self, image: Image, max_size: i32) -> Result<Vec<Color>, String> {
        RtexturesImpl::__load_image_pallete(image, max_size)
    }

    /// Unload color data loaded with LoadImageColors()
    fn unload_image_colors(&self, colors: *mut Color) {
        RtexturesImpl::__unload_image_colors(colors)
    }

    /// Get image alpha border rectangle
    fn get_image_alpha_border(&self, image: Image, threshold: f32) -> Rectangle {
        RtexturesImpl::__get_image_alpha_border(image, threshold)
    }

    /// Get image pixel color at (x, y) position
    fn get_image_color(&self, image: Image, x: i32, y: i32) -> Color {
        RtexturesImpl::__get_image_color(image, x, y)
    }

    // Image drawing methods

    /// Clear image background with given color
    fn image_clear_background(&self, image: &mut Image, color: Color) {
        RtexturesImpl::__image_clear_background(image, color)
    }

    /// Draw pixel within an image
    fn image_draw_pixel(&self, image: &mut Image, x: i32, y: i32, color: Color) {
        RtexturesImpl::__image_draw_pixel(image, x, y, color)
    }

    /// Draw pixel within an image (Vector version)
    fn image_draw_pixel_v(&self, image: &mut Image, position: Vector2, color: Color) {
        RtexturesImpl::__image_draw_pixel_v(image, position, color)
    }

    /// Draw line within an image
    fn image_draw_line(
        &self,
        image: &mut Image,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_line(image, start_x, start_y, end_x, end_y, color)
    }

    /// Draw line within an image (Vector version)
    fn image_draw_line_v(&self, image: &mut Image, start: Vector2, end: Vector2, color: Color) {
        RtexturesImpl::__image_draw_line_v(image, start, end, color)
    }

    /// Draw a filled circle within an image
    fn image_draw_circle(
        &self,
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_circle(image, center_x, center_y, radius, color)
    }

    /// Draw a filled circle within an image (Vector version)
    fn image_draw_circle_v(&self, image: &mut Image, center: Vector2, radius: i32, color: Color) {
        RtexturesImpl::__image_draw_circle_v(image, center, radius, color)
    }

    /// Draw circle outline within an image
    fn image_draw_circle_lines(
        &self,
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_circle_lines(image, center_x, center_y, radius, color)
    }

    /// Draw circle outline within an image (Vector version)
    fn image_draw_circle_lines_v(
        &self,
        image: &mut Image,
        center: Vector2,
        radius: i32,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_circle_lines_v(image, center, radius, color)
    }

    /// Draw rectangle within an image
    fn image_draw_rectangle(
        &self,
        image: &mut Image,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_rectangle(image, x, y, width, height, color)
    }

    /// Draw rectangle within an image (Vector version)
    fn image_draw_rectangle_v(
        &self,
        image: &mut Image,
        position: Vector2,
        size: Vector2,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_rectangle_v(image, position, size, color)
    }

    /// Draw rectangle within an image
    fn image_draw_rectangle_rec(&self, image: &mut Image, rec: Rectangle, color: Color) {
        RtexturesImpl::__image_draw_rectangle_rec(image, rec, color)
    }

    /// Draw rectangle lines within an image
    fn image_draw_rectangle_lines(
        &self,
        image: &mut Image,
        rec: Rectangle,
        thick: i32,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_rectangle_lines(image, rec, thick, color)
    }

    /// Draw a source image within a destination image (tint applied to source)
    fn image_draw(
        &self,
        image: &mut Image,
        src: Image,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: Color,
    ) {
        RtexturesImpl::__image_draw(image, src, src_rec, dst_rec, tint)
    }

    /// Draw text (using default font) within an image (destination)
    fn image_draw_text(
        &self,
        image: &mut Image,
        text: impl Display,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) {
        RtexturesImpl::__image_draw_text(image, text, x, y, font_size, color)
    }

    /// Draw text (custom sprite font) within an image (destination)
    fn image_draw_text_ex(
        &self,
        image: &mut Image,
        font: Font,
        text: impl Display,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        RtexturesImpl::__image_draw_text_ex(image, font, text, position, font_size, spacing, tint)
    }

    // Texture loading methods

    /// Load texture from file into GPU memory (VRAM)
    fn load_texture(&self, filename: impl Display) -> Result<Texture2D, String> {
        RtexturesImpl::__load_texture(filename)
    }

    /// Load texture from image data
    fn load_texture_from_image(&self, image: Image) -> Result<Texture2D, String> {
        RtexturesImpl::__load_texture_from_image(image)
    }

    /// Load cubemap from image, multiple image cubemap layouts supported
    fn load_texture_cubemap(
        &self,
        image: Image,
        layout: CubemapLayout,
    ) -> Result<TextureCubemap, String> {
        RtexturesImpl::__load_texture_cubemap(image, layout)
    }

    /// Load texture for rendering (framebuffer)
    fn load_render_texture(&self, width: i32, height: i32) -> RenderTexture2D {
        RtexturesImpl::__load_render_texture(width, height)
    }

    /// Check whether a texture is ready
    fn is_texture_ready(&self, texture: Texture2D) -> bool {
        RtexturesImpl::__is_texture_ready(texture)
    }

    /// Unload texture from GPU memory (VRAM)
    fn unload_texture(&self, texture: Texture2D) {
        RtexturesImpl::__unload_texture(texture)
    }

    /// Check whether a render texture is ready
    fn is_render_texture_ready(&self, target: RenderTexture2D) -> bool {
        RtexturesImpl::__is_render_texture_ready(target)
    }

    /// Unload render texture from GPU memory (VRAM)
    fn unload_render_texture(&self, target: RenderTexture2D) {
        RtexturesImpl::__unload_render_texture(target)
    }

    /// Update GPU texture with new data
    fn update_texture(&self, texture: Texture2D, pixels: &[u8]) {
        RtexturesImpl::__update_texture(texture, pixels)
    }

    /// Update GPU texture rectangle with new data
    fn update_texture_rec(&self, texture: Texture2D, rec: Rectangle, pixels: &[u8]) {
        RtexturesImpl::__update_texture_rec(texture, rec, pixels)
    }

    // Texture configuration methods

    /// Generate GPU mipmaps for a texture
    fn gen_texture_mipmaps(&self, texture: &mut Texture2D) {
        RtexturesImpl::__gen_texture_mipmaps(texture)
    }

    /// Set texture scaling filter mode
    fn set_texture_filter(&self, texture: Texture2D, filter: TextureFilter) {
        RtexturesImpl::__set_texture_filter(texture, filter)
    }

    /// Set texture wrapping mode
    fn set_texture_wrap(&self, texture: Texture2D, wrap: TextureWrap) {
        RtexturesImpl::__set_texture_wrap(texture, wrap)
    }

    // Texture drawing methods

    // Draw a Texture2D
    fn draw_texture(&self, texture: Texture2D, x: i32, y: i32, tint: Color) {
        RtexturesImpl::__draw_texture(texture, x, y, tint)
    }

    /// Draw a Texture2D with position defined as Vector2
    fn draw_texture_v(&self, texture: Texture2D, position: Vector2, tint: Color) {
        RtexturesImpl::__draw_texture_v(texture, position, tint)
    }

    /// Draw a Texture2D with extended parameters
    fn draw_texture_ex(
        &self,
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        RtexturesImpl::__draw_texture_ex(texture, position, rotation, scale, tint)
    }

    /// Draw a part of a texture defined by a rectangle
    fn draw_texture_rec(
        &self,
        texture: Texture2D,
        source: Rectangle,
        position: Vector2,
        tint: Color,
    ) {
        RtexturesImpl::__draw_texture_rec(texture, source, position, tint)
    }

    /// Draw a part of a texture defined by a rectangle with pro parameters
    fn draw_texture_pro(
        &self,
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        RtexturesImpl::__draw_texture_pro(texture, source, dest, origin, rotation, tint)
    }

    /// Draws a texture (or part of it) that stretches or shrinks nicely
    fn draw_texture_n_patch(
        &self,
        texture: Texture2D,
        info: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        RtexturesImpl::__draw_texture_n_patch(texture, info, dest, origin, rotation, tint)
    }

    // Color/pixel related methods

    /// Get color with alpha applied, alpha goes from 0.0 to 1.0
    fn fade(&self, color: Color, alpha: f32) -> Color {
        RtexturesImpl::__fade(color, alpha)
    }

    /// Get hexadecimal value for a Color
    fn color_to_int(&self, color: Color) -> i32 {
        RtexturesImpl::__color_to_int(color)
    }

    /// Get Color normalized as float [0..1]
    fn color_normalize(&self, color: Color) -> Vector4 {
        RtexturesImpl::__color_normalize(color)
    }

    /// Get Color from normalized values [0..1]
    fn color_from_normalized(&self, normalized: Vector4) -> Color {
        RtexturesImpl::__color_from_normalized(normalized)
    }

    /// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
    fn color_to_hsv(&self, color: Color) -> Vector3 {
        RtexturesImpl::__color_to_hsv(color)
    }

    /// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
    fn color_from_hsv(&self, hue: f32, saturation: f32, value: f32) -> Color {
        RtexturesImpl::__color_from_hsv(hue, saturation, value)
    }

    /// Get color multiplied with another color
    fn color_tint(&self, color: Color, tint: Color) -> Color {
        RtexturesImpl::__color_tint(color, tint)
    }

    /// Get color with brightness correction, brightness factor goes from -1.0 to 1.0
    fn color_brightness(&self, color: Color, factor: f32) -> Color {
        RtexturesImpl::__color_brightness(color, factor)
    }

    /// Get color with contrast correction, contrast values between -1.0 and 1.0
    fn color_contrast(&self, color: Color, contrast: f32) -> Color {
        RtexturesImpl::__color_contrast(color, contrast)
    }

    /// Get color with alpha applied, alpha goes from 0.0 to 1.0
    fn color_alpha(&self, color: Color, alpha: f32) -> Color {
        RtexturesImpl::__color_alpha(color, alpha)
    }

    /// Get src alpha-blended into dst color with tint
    fn color_alpha_blend(&self, dst: Color, src: Color, tint: Color) -> Color {
        RtexturesImpl::__color_alpha_blend(dst, src, tint)
    }

    /// Get Color structure from hexadecimal value
    fn get_color(&self, hex_value: u32) -> Color {
        RtexturesImpl::__get_color(hex_value)
    }

    /// Get Color from a source pixel pointer of certain format
    fn get_pixel_color(&self, ptr: &mut Vec<u8>, format: PixelFormat) -> Color {
        RtexturesImpl::__get_pixel_color(ptr, format)
    }

    /// Set color formatted into destination pixel pointer
    fn set_pixel_color(&self, ptr: &mut Vec<u8>, color: Color, format: PixelFormat) {
        RtexturesImpl::__set_pixel_color(ptr, color, format)
    }

    /// Get pixel data size in bytes for certain format
    fn get_pixel_data_size(&self, width: i32, height: i32, format: impl Into<usize>) -> i32 {
        RtexturesImpl::__get_pixel_data_size(width, height, format)
    }
}
