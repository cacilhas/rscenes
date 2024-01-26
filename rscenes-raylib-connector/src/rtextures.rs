use raylib_ffi::{enums::*, *};
use std::{
    f32::consts::PI,
    ffi::{c_uchar, c_void},
    fmt::Display,
    slice,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rtextures;

/// Crate only methods
impl Rtextures {
    // Image loading

    pub(crate) fn __load_image(filename: impl Display) -> Image {
        unsafe { LoadImage(rl_str!(filename)) }
    }

    pub(crate) fn __load_image_raw(
        filename: impl Display,
        width: i32,
        height: i32,
        format: impl Into<usize>,
        header_size: i32,
    ) -> Image {
        unsafe {
            LoadImageRaw(
                rl_str!(filename),
                width,
                height,
                format.into() as i32,
                header_size,
            )
        }
    }

    pub(crate) fn __load_image_svg(
        filename_or_string: impl Display,
        width: i32,
        height: i32,
    ) -> Image {
        unsafe { LoadImageSvg(rl_str!(filename_or_string), width, height) }
    }

    pub(crate) fn __load_image_anim(filename: impl Display) -> (Image, i32) {
        unsafe {
            let mut frames: i32 = 0;
            let img = LoadImageAnim(rl_str!(filename), &mut frames);
            (img, frames)
        }
    }

    pub(crate) fn __load_image_from_memory(tpe: impl Display, data: Vec<u8>) -> Image {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_ptr() as *mut c_uchar;
            LoadImageFromMemory(rl_str!(tpe), data, size)
        }
    }

    pub(crate) fn __load_image_from_texture(texture: Texture2D) -> Image {
        unsafe { LoadImageFromTexture(texture) }
    }

    pub(crate) fn __load_image_from_screen() -> Image {
        unsafe { LoadImageFromScreen() }
    }

    pub(crate) fn __is_image_ready(image: Image) -> bool {
        unsafe { IsImageReady(image) }
    }

    pub(crate) fn __unload_image(image: Image) {
        unsafe { UnloadImage(image) }
    }

    pub(crate) fn __export_image(image: Image, filename: impl Display) -> bool {
        unsafe { ExportImage(image, rl_str!(filename)) }
    }

    pub(crate) fn __export_image_to_memory(image: Image, tpe: impl Display) -> Vec<u8> {
        unsafe {
            let mut size: i32 = 0;
            let res = ExportImageToMemory(image, rl_str!(tpe), &mut size);
            slice::from_raw_parts(res, size as usize).to_vec()
        }
    }

    pub(crate) fn __export_image_as_code(image: Image, filename: impl Display) -> bool {
        unsafe { ExportImageAsCode(image, rl_str!(filename)) }
    }

    // Image generation methods

    pub(crate) fn __gen_image_color(width: i32, height: i32, color: Color) -> Image {
        unsafe { GenImageColor(width, height, color) }
    }

    pub(crate) fn __gen_image_gradient_linear(
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

    pub(crate) fn __gen_image_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        unsafe { GenImageGradientRadial(width, height, density, inner, outer) }
    }

    pub(crate) fn __gen_image_gradient_square(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        unsafe { GenImageGradientSquare(width, height, density, inner, outer) }
    }

    pub(crate) fn __gen_image_checked(
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: Color,
        col2: Color,
    ) -> Image {
        unsafe { GenImageChecked(width, height, checks_x, checks_y, col1, col2) }
    }

    pub(crate) fn __gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
        unsafe { GenImageWhiteNoise(width, height, factor) }
    }

    pub(crate) fn __gen_image_perlin_noise(
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Image {
        unsafe { GenImagePerlinNoise(width, height, offset_x, offset_y, scale) }
    }

    pub(crate) fn __gen_image_cellular(width: i32, height: i32, tile_size: i32) -> Image {
        unsafe { GenImageCellular(width, height, tile_size) }
    }

    pub(crate) fn __gen_image_text(width: i32, height: i32, text: impl Display) -> Image {
        unsafe { GenImageText(width, height, rl_str!(text)) }
    }

    // Image manipulation methods

    pub(crate) fn __image_copy(image: Image) -> Image {
        unsafe { ImageCopy(image) }
    }

    pub(crate) fn __image_from_image(image: Image, rec: Rectangle) -> Image {
        unsafe { ImageFromImage(image, rec) }
    }

    pub(crate) fn __image_text(text: impl Display, font_size: i32, color: Color) -> Image {
        unsafe { ImageText(rl_str!(text), font_size, color) }
    }

    pub(crate) fn __image_text_ex(
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Image {
        unsafe { ImageTextEx(font, rl_str!(text), font_size, spacing, tint) }
    }

    pub(crate) fn __image_format(image: &mut Image, format: impl Into<usize>) {
        unsafe { ImageFormat(image, format.into() as i32) }
    }

    pub(crate) fn __image_to_pot(image: &mut Image, fill: Color) {
        unsafe { ImageToPOT(image, fill) }
    }

    pub(crate) fn __image_crop(image: &mut Image, crop: Rectangle) {
        unsafe { ImageCrop(image, crop) }
    }

    pub(crate) fn __image_alpha_crop(image: &mut Image, threshold: f32) {
        unsafe { ImageAlphaCrop(image, threshold) }
    }

    pub(crate) fn __image_alpha_clear(image: &mut Image, color: Color, threshold: f32) {
        unsafe { ImageAlphaClear(image, color, threshold) }
    }

    pub(crate) fn __image_alpha_mask(image: &mut Image, alpha_mask: Image) {
        unsafe { ImageAlphaMask(image, alpha_mask) }
    }

    pub(crate) fn __image_alpha_premultiply(image: &mut Image) {
        unsafe { ImageAlphaPremultiply(image) }
    }

    pub(crate) fn __image_blur_gaussian(image: &mut Image, blur_size: i32) {
        unsafe { ImageBlurGaussian(image, blur_size) }
    }

    pub(crate) fn __image_resize(image: &mut Image, width: i32, height: i32) {
        unsafe { ImageResize(image, width, height) }
    }

    pub(crate) fn __image_resize_nn(image: &mut Image, width: i32, height: i32) {
        unsafe { ImageResizeNN(image, width, height) }
    }

    pub(crate) fn __image_resize_canvas(
        image: &mut Image,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) {
        unsafe { ImageResizeCanvas(image, width, height, offset_x, offset_y, fill) }
    }

    pub(crate) fn __image_mipmaps(image: &mut Image) {
        unsafe { ImageMipmaps(image) }
    }

    pub(crate) fn __image_dither(image: &mut Image, r: i32, g: i32, b: i32, a: i32) {
        unsafe { ImageDither(image, r, g, b, a) }
    }

    pub(crate) fn __image_flip_vertical(image: &mut Image) {
        unsafe { ImageFlipVertical(image) }
    }

    pub(crate) fn __image_flip_horizontal(image: &mut Image) {
        unsafe { ImageFlipHorizontal(image) }
    }

    pub(crate) fn __image_rotate(image: &mut Image, angle: f32) {
        unsafe {
            let degrees = (angle * 180.0 / PI) as i32;
            ImageRotate(image, degrees)
        }
    }

    pub(crate) fn __image_rotate_cw(image: &mut Image) {
        unsafe { ImageRotateCW(image) }
    }

    pub(crate) fn __image_rotate_ccw(image: &mut Image) {
        unsafe { ImageRotateCCW(image) }
    }

    pub(crate) fn __image_color_tint(image: &mut Image, tint: Color) {
        unsafe { ImageColorTint(image, tint) }
    }

    pub(crate) fn __image_color_invert(image: &mut Image) {
        unsafe { ImageColorInvert(image) }
    }

    pub(crate) fn __image_color_grayscale(image: &mut Image) {
        unsafe { ImageColorGrayscale(image) }
    }

    pub(crate) fn __image_color_contrast(image: &mut Image, contrast: f32) {
        unsafe { ImageColorContrast(image, contrast) }
    }

    pub(crate) fn __image_color_brightness(image: &mut Image, brightness: i32) {
        unsafe { ImageColorBrightness(image, brightness) }
    }

    pub(crate) fn __image_color_replace(image: &mut Image, color: Color, replace: Color) {
        unsafe { ImageColorReplace(image, color, replace) }
    }

    pub(crate) fn __load_image_colors(image: Image) -> *mut Color {
        unsafe { LoadImageColors(image) }
    }

    pub(crate) fn __load_image_pallete(image: Image, max_size: i32) -> Vec<Color> {
        unsafe {
            let mut size: i32 = 0;
            let res = LoadImagePalette(image, max_size, &mut size);
            slice::from_raw_parts(res, size as usize).to_vec()
        }
    }

    pub(crate) fn __unload_image_colors(colors: *mut Color) {
        unsafe { UnloadImageColors(colors) }
    }

    pub(crate) fn __unload_image_palette(colors: Vec<Color>) {
        unsafe { UnloadImagePalette(colors.as_ptr() as *mut Color) }
    }

    pub(crate) fn __get_image_alpha_border(image: Image, threshold: f32) -> Rectangle {
        unsafe { GetImageAlphaBorder(image, threshold) }
    }

    pub(crate) fn __get_image_color(image: Image, x: i32, y: i32) -> Color {
        unsafe { GetImageColor(image, x, y) }
    }

    // Image drawing methods

    pub(crate) fn __image_clear_background(image: &mut Image, color: Color) {
        unsafe { ImageClearBackground(image, color) }
    }

    pub(crate) fn __image_draw_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
        unsafe { ImageDrawPixel(image, x, y, color) }
    }

    pub(crate) fn __image_draw_pixel_v(image: &mut Image, position: Vector2, color: Color) {
        unsafe { ImageDrawPixelV(image, position, color) }
    }

    pub(crate) fn __image_draw_line(
        image: &mut Image,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Color,
    ) {
        unsafe { ImageDrawLine(image, start_x, start_y, end_x, end_y, color) }
    }

    pub(crate) fn __image_draw_line_v(
        image: &mut Image,
        start: Vector2,
        end: Vector2,
        color: Color,
    ) {
        unsafe { ImageDrawLineV(image, start, end, color) }
    }

    pub(crate) fn __image_draw_circle(
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        unsafe { ImageDrawCircle(image, center_x, center_y, radius, color) }
    }

    pub(crate) fn __image_draw_circle_v(
        image: &mut Image,
        center: Vector2,
        radius: i32,
        color: Color,
    ) {
        unsafe { ImageDrawCircleV(image, center, radius, color) }
    }

    pub(crate) fn __image_draw_circle_lines(
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        unsafe { ImageDrawCircleLines(image, center_x, center_y, radius, color) }
    }

    pub(crate) fn __image_draw_circle_lines_v(
        image: &mut Image,
        center: Vector2,
        radius: i32,
        color: Color,
    ) {
        unsafe { ImageDrawCircleLinesV(image, center, radius, color) }
    }

    pub(crate) fn __image_draw_rectangle(
        image: &mut Image,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) {
        unsafe { ImageDrawRectangle(image, x, y, width, height, color) }
    }

    pub(crate) fn __image_draw_rectangle_v(
        image: &mut Image,
        position: Vector2,
        size: Vector2,
        color: Color,
    ) {
        unsafe { ImageDrawRectangleV(image, position, size, color) }
    }

    pub(crate) fn __image_draw_rectangle_rec(image: &mut Image, rec: Rectangle, color: Color) {
        unsafe { ImageDrawRectangleRec(image, rec, color) }
    }

    pub(crate) fn __image_draw_rectangle_lines(
        image: &mut Image,
        rec: Rectangle,
        thick: i32,
        color: Color,
    ) {
        unsafe { ImageDrawRectangleLines(image, rec, thick, color) }
    }

    pub(crate) fn __image_draw(
        image: &mut Image,
        src: Image,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: Color,
    ) {
        unsafe { ImageDraw(image, src, src_rec, dst_rec, tint) }
    }

    pub(crate) fn __image_draw_text(
        image: &mut Image,
        text: impl Display,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) {
        unsafe { ImageDrawText(image, rl_str!(text), x, y, font_size, color) }
    }

    pub(crate) fn __image_draw_text_ex(
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

    pub(crate) fn __load_texture(filename: impl Display) -> Texture2D {
        unsafe { LoadTexture(rl_str!(filename)) }
    }

    pub(crate) fn __load_texture_from_image(image: Image) -> Texture2D {
        unsafe { LoadTextureFromImage(image) }
    }

    pub(crate) fn __load_texture_cubemap(image: Image, layout: impl Into<usize>) -> TextureCubemap {
        unsafe { LoadTextureCubemap(image, layout.into() as i32) }
    }

    pub(crate) fn __load_render_texture(width: i32, height: i32) -> RenderTexture2D {
        unsafe { LoadRenderTexture(width, height) }
    }

    pub(crate) fn __is_texture_ready(texture: Texture2D) -> bool {
        unsafe { IsTextureReady(texture) }
    }

    pub(crate) fn __unload_texture(texture: Texture2D) {
        unsafe { UnloadTexture(texture) }
    }

    pub(crate) fn __is_render_texture_ready(target: RenderTexture2D) -> bool {
        unsafe { IsRenderTextureReady(target) }
    }

    pub(crate) fn __unload_render_texture(target: RenderTexture2D) {
        unsafe { UnloadRenderTexture(target) }
    }

    pub(crate) fn __update_texture(texture: Texture2D, pixels: Vec<u8>) {
        unsafe { UpdateTexture(texture, pixels.as_ptr() as *const c_void) }
    }

    pub(crate) fn __update_texture_rec(texture: Texture2D, rec: Rectangle, pixels: Vec<u8>) {
        unsafe { UpdateTextureRec(texture, rec, pixels.as_ptr() as *const c_void) }
    }

    // Texture configuration methods

    pub(crate) fn __gen_texture_mipmaps(texture: &mut Texture2D) {
        unsafe { GenTextureMipmaps(texture) }
    }

    pub(crate) fn __set_texture_filter(texture: Texture2D, filter: impl Into<usize>) {
        unsafe { SetTextureFilter(texture, filter.into() as i32) }
    }

    pub(crate) fn __set_texture_wrap(texture: Texture2D, wrap: impl Into<usize>) {
        unsafe { SetTextureWrap(texture, wrap.into() as i32) }
    }

    // Texture drawing methods

    pub(crate) fn __draw_texture(texture: Texture2D, x: i32, y: i32, tint: Color) {
        unsafe { DrawTexture(texture, x, y, tint) }
    }

    pub(crate) fn __draw_texture_v(texture: Texture2D, position: Vector2, tint: Color) {
        unsafe { DrawTextureV(texture, position, tint) }
    }

    pub(crate) fn __draw_texture_ex(
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        unsafe { DrawTextureEx(texture, position, rotation, scale, tint) }
    }

    pub(crate) fn __draw_texture_rec(
        texture: Texture2D,
        source: Rectangle,
        position: Vector2,
        tint: Color,
    ) {
        unsafe { DrawTextureRec(texture, source, position, tint) }
    }

    pub(crate) fn __draw_texture_pro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe { DrawTexturePro(texture, source, dest, origin, rotation, tint) }
    }

    pub(crate) fn __draw_texture_n_patch(
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

    pub(crate) fn __fade(color: Color, alpha: f32) -> Color {
        unsafe { Fade(color, alpha) }
    }

    pub(crate) fn __color_to_int(color: Color) -> i32 {
        unsafe { ColorToInt(color) }
    }

    pub(crate) fn __color_normalize(color: Color) -> Vector4 {
        unsafe { ColorNormalize(color) }
    }

    pub(crate) fn __color_from_normalized(normalized: Vector4) -> Color {
        unsafe { ColorFromNormalized(normalized) }
    }

    pub(crate) fn __color_to_hsv(color: Color) -> Vector3 {
        unsafe { ColorToHSV(color) }
    }

    pub(crate) fn __color_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        unsafe { ColorFromHSV(hue, saturation, value) }
    }

    pub(crate) fn __color_tint(color: Color, tint: Color) -> Color {
        unsafe { ColorTint(color, tint) }
    }

    pub(crate) fn __color_brightness(color: Color, factor: f32) -> Color {
        unsafe { ColorBrightness(color, factor) }
    }

    pub(crate) fn __color_contrast(color: Color, contrast: f32) -> Color {
        unsafe { ColorContrast(color, contrast) }
    }

    pub(crate) fn __color_alpha(color: Color, alpha: f32) -> Color {
        unsafe { ColorAlpha(color, alpha) }
    }

    pub(crate) fn __color_alpha_blend(dst: Color, src: Color, tint: Color) -> Color {
        unsafe { ColorAlphaBlend(dst, src, tint) }
    }

    pub(crate) fn __get_color(hex_value: u32) -> Color {
        unsafe { GetColor(hex_value) }
    }

    pub(crate) fn __get_pixel_color(ptr: Vec<u8>, format: impl Into<usize>) -> Color {
        unsafe { GetPixelColor(ptr.as_ptr() as *mut c_void, format.into() as i32) }
    }

    pub(crate) fn __set_pixel_color(ptr: &mut Vec<u8>, color: Color, format: impl Into<usize>) {
        unsafe { SetPixelColor(ptr.as_ptr() as *mut c_void, color, format.into() as i32) }
    }

    pub(crate) fn __get_pixel_data_size(width: i32, height: i32, format: impl Into<usize>) -> i32 {
        unsafe { GetPixelDataSize(width, height, format.into() as i32) }
    }
}

/// Exported methods
impl Rtextures {
    // Image loading

    pub fn load_image(&self, filename: impl Display) -> Image {
        Self::__load_image(filename)
    }

    pub fn load_image_raw(
        &self,
        filename: impl Display,
        width: i32,
        height: i32,
        format: PixelFormat,
        header_size: i32,
    ) -> Image {
        Self::__load_image_raw(filename, width, height, format, header_size)
    }

    pub fn load_image_svg(
        &self,
        filename_or_string: impl Display,
        width: i32,
        height: i32,
    ) -> Image {
        Self::__load_image_svg(filename_or_string, width, height)
    }

    pub fn load_image_anim(&self, filename: impl Display) -> (Image, i32) {
        Self::__load_image_anim(filename)
    }

    pub fn load_image_from_memory(&self, tpe: impl Display, data: Vec<u8>) -> Image {
        Self::__load_image_from_memory(tpe, data)
    }

    pub fn load_image_from_texture(&self, texture: Texture2D) -> Image {
        Self::__load_image_from_texture(texture)
    }

    pub fn load_image_from_screen(&self) -> Image {
        Self::__load_image_from_screen()
    }

    pub fn is_image_ready(&self, image: Image) -> bool {
        Self::__is_image_ready(image)
    }

    pub fn unload_image(&self, image: Image) {
        Self::__unload_image(image)
    }

    pub fn export_image(&self, image: Image, filename: impl Display) -> bool {
        Self::__export_image(image, filename)
    }

    pub fn export_image_to_memory(&self, image: Image, tpe: impl Display) -> Vec<u8> {
        Self::__export_image_to_memory(image, tpe)
    }

    pub fn export_image_as_code(&self, image: Image, filename: impl Display) -> bool {
        Self::__export_image_as_code(image, filename)
    }

    // Image generation methods

    pub fn gen_image_color(&self, width: i32, height: i32, color: Color) -> Image {
        Self::__gen_image_color(width, height, color)
    }

    pub fn gen_image_gradient_linear(
        &self,
        width: i32,
        height: i32,
        angle: f32,
        start: Color,
        end: Color,
    ) -> Image {
        Self::__gen_image_gradient_linear(width, height, angle, start, end)
    }

    pub fn gen_image_gradient_radial(
        &self,
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        Self::__gen_image_gradient_radial(width, height, density, inner, outer)
    }

    pub fn gen_image_gradient_square(
        &self,
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        Self::__gen_image_gradient_square(width, height, density, inner, outer)
    }

    pub fn gen_image_checked(
        &self,
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: Color,
        col2: Color,
    ) -> Image {
        Self::__gen_image_checked(width, height, checks_x, checks_y, col1, col2)
    }

    pub fn gen_image_white_noise(&self, width: i32, height: i32, factor: f32) -> Image {
        Self::__gen_image_white_noise(width, height, factor)
    }

    pub fn gen_image_perlin_noise(
        &self,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Image {
        Self::__gen_image_perlin_noise(width, height, offset_x, offset_y, scale)
    }

    pub fn gen_image_cellular(&self, width: i32, height: i32, tile_size: i32) -> Image {
        Self::__gen_image_cellular(width, height, tile_size)
    }

    pub fn gen_image_text(&self, width: i32, height: i32, text: impl Display) -> Image {
        Self::__gen_image_text(width, height, text)
    }

    // Image manipulation methods

    pub fn image_copy(&self, image: Image) -> Image {
        Self::__image_copy(image)
    }

    pub fn image_from_image(&self, image: Image, rec: Rectangle) -> Image {
        Self::__image_from_image(image, rec)
    }

    pub fn image_text(&self, text: impl Display, font_size: i32, color: Color) -> Image {
        Self::__image_text(text, font_size, color)
    }

    pub fn image_text_ex(
        &self,
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Image {
        Self::__image_text_ex(font, text, font_size, spacing, tint)
    }

    pub fn image_format(&self, image: &mut Image, format: PixelFormat) {
        Self::__image_format(image, format)
    }

    pub fn image_to_pot(&self, image: &mut Image, fill: Color) {
        Self::__image_to_pot(image, fill)
    }

    pub fn image_crop(&self, image: &mut Image, crop: Rectangle) {
        Self::__image_crop(image, crop)
    }

    pub fn image_alpha_crop(&self, image: &mut Image, threshold: f32) {
        Self::__image_alpha_crop(image, threshold)
    }

    pub fn image_alpha_clear(&self, image: &mut Image, color: Color, threshold: f32) {
        Self::__image_alpha_clear(image, color, threshold)
    }

    pub fn image_alpha_mask(&self, image: &mut Image, alpha_mask: Image) {
        Self::__image_alpha_mask(image, alpha_mask)
    }

    pub fn image_alpha_premultiply(&self, image: &mut Image) {
        Self::__image_alpha_premultiply(image)
    }

    pub fn image_blur_gaussian(&self, image: &mut Image, blur_size: i32) {
        Self::__image_blur_gaussian(image, blur_size)
    }

    pub fn image_resize(&self, image: &mut Image, width: i32, height: i32) {
        Self::__image_resize(image, width, height)
    }

    pub fn image_resize_nn(&self, image: &mut Image, width: i32, height: i32) {
        Self::__image_resize_nn(image, width, height)
    }

    pub fn image_resize_canvas(
        &self,
        image: &mut Image,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) {
        Self::__image_resize_canvas(image, width, height, offset_x, offset_y, fill)
    }

    pub fn image_mipmaps(&self, image: &mut Image) {
        Self::__image_mipmaps(image)
    }

    pub fn image_dither(&self, image: &mut Image, r: i32, g: i32, b: i32, a: i32) {
        Self::__image_dither(image, r, g, b, a)
    }

    pub fn image_flip_vertical(&self, image: &mut Image) {
        Self::__image_flip_vertical(image)
    }

    pub fn image_flip_horizontal(&self, image: &mut Image) {
        Self::__image_flip_horizontal(image)
    }

    pub fn image_rotate(&self, image: &mut Image, angle: f32) {
        Self::__image_rotate(image, angle)
    }

    pub fn image_rotate_cw(&self, image: &mut Image) {
        Self::__image_rotate_cw(image)
    }

    pub fn image_rotate_ccw(&self, image: &mut Image) {
        Self::__image_rotate_ccw(image)
    }

    pub fn image_color_tint(&self, image: &mut Image, tint: Color) {
        Self::__image_color_tint(image, tint)
    }

    pub fn image_color_invert(&self, image: &mut Image) {
        Self::__image_color_invert(image)
    }

    pub fn image_color_grayscale(&self, image: &mut Image) {
        Self::__image_color_grayscale(image)
    }

    pub fn image_color_contrast(&self, image: &mut Image, contrast: f32) {
        Self::__image_color_contrast(image, contrast)
    }

    pub fn image_color_brightness(&self, image: &mut Image, brightness: i32) {
        Self::__image_color_brightness(image, brightness)
    }

    pub fn image_color_replace(&self, image: &mut Image, color: Color, replace: Color) {
        Self::__image_color_replace(image, color, replace)
    }

    pub fn load_image_colors(&self, image: Image) -> *mut Color {
        Self::__load_image_colors(image)
    }

    pub fn load_image_pallete(&self, image: Image, max_size: i32) -> Vec<Color> {
        Self::__load_image_pallete(image, max_size)
    }

    pub fn unload_image_colors(&self, colors: *mut Color) {
        Self::__unload_image_colors(colors)
    }

    pub fn unload_image_palette(&self, colors: Vec<Color>) {
        Self::__unload_image_palette(colors)
    }

    pub fn get_image_alpha_border(&self, image: Image, threshold: f32) -> Rectangle {
        Self::__get_image_alpha_border(image, threshold)
    }

    pub fn get_image_color(&self, image: Image, x: i32, y: i32) -> Color {
        Self::__get_image_color(image, x, y)
    }

    // Image drawing methods

    pub fn image_clear_background(&self, image: &mut Image, color: Color) {
        Self::__image_clear_background(image, color)
    }

    pub fn image_draw_pixel(&self, image: &mut Image, x: i32, y: i32, color: Color) {
        Self::__image_draw_pixel(image, x, y, color)
    }

    pub fn image_draw_pixel_v(&self, image: &mut Image, position: Vector2, color: Color) {
        Self::__image_draw_pixel_v(image, position, color)
    }

    pub fn image_draw_line(
        &self,
        image: &mut Image,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Color,
    ) {
        Self::__image_draw_line(image, start_x, start_y, end_x, end_y, color)
    }

    pub fn image_draw_line_v(&self, image: &mut Image, start: Vector2, end: Vector2, color: Color) {
        Self::__image_draw_line_v(image, start, end, color)
    }

    pub fn image_draw_circle(
        &self,
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        Self::__image_draw_circle(image, center_x, center_y, radius, color)
    }

    pub fn image_draw_circle_v(
        &self,
        image: &mut Image,
        center: Vector2,
        radius: i32,
        color: Color,
    ) {
        Self::__image_draw_circle_v(image, center, radius, color)
    }

    pub fn image_draw_circle_lines(
        &self,
        image: &mut Image,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) {
        Self::__image_draw_circle_lines(image, center_x, center_y, radius, color)
    }

    pub fn image_draw_circle_lines_v(
        &self,
        image: &mut Image,
        center: Vector2,
        radius: i32,
        color: Color,
    ) {
        Self::__image_draw_circle_lines_v(image, center, radius, color)
    }

    pub fn image_draw_rectangle(
        &self,
        image: &mut Image,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) {
        Self::__image_draw_rectangle(image, x, y, width, height, color)
    }

    pub fn image_draw_rectangle_v(
        &self,
        image: &mut Image,
        position: Vector2,
        size: Vector2,
        color: Color,
    ) {
        Self::__image_draw_rectangle_v(image, position, size, color)
    }

    pub fn image_draw_rectangle_rec(&self, image: &mut Image, rec: Rectangle, color: Color) {
        Self::__image_draw_rectangle_rec(image, rec, color)
    }

    pub fn image_draw_rectangle_lines(
        &self,
        image: &mut Image,
        rec: Rectangle,
        thick: i32,
        color: Color,
    ) {
        Self::__image_draw_rectangle_lines(image, rec, thick, color)
    }

    pub fn image_draw(
        &self,
        image: &mut Image,
        src: Image,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: Color,
    ) {
        Self::__image_draw(image, src, src_rec, dst_rec, tint)
    }

    pub fn image_draw_text(
        &self,
        image: &mut Image,
        text: impl Display,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) {
        Self::__image_draw_text(image, text, x, y, font_size, color)
    }

    pub fn image_draw_text_ex(
        &self,
        image: &mut Image,
        font: Font,
        text: impl Display,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        Self::__image_draw_text_ex(image, font, text, position, font_size, spacing, tint)
    }

    // Texture loading methods

    pub fn load_texture(&self, filename: impl Display) -> Texture2D {
        Self::__load_texture(filename)
    }

    pub fn load_texture_from_image(&self, image: Image) -> Texture2D {
        Self::__load_texture_from_image(image)
    }

    pub fn load_texture_cubemap(&self, image: Image, layout: CubemapLayout) -> TextureCubemap {
        Self::__load_texture_cubemap(image, layout)
    }

    pub fn load_render_texture(&self, width: i32, height: i32) -> RenderTexture2D {
        Self::__load_render_texture(width, height)
    }

    pub fn is_texture_ready(&self, texture: Texture2D) -> bool {
        Self::__is_texture_ready(texture)
    }

    pub fn unload_texture(&self, texture: Texture2D) {
        Self::__unload_texture(texture)
    }

    pub fn is_render_texture_ready(&self, target: RenderTexture2D) -> bool {
        Self::__is_render_texture_ready(target)
    }

    pub fn unload_render_texture(&self, target: RenderTexture2D) {
        Self::__unload_render_texture(target)
    }

    pub fn update_texture(&self, texture: Texture2D, pixels: Vec<u8>) {
        Self::__update_texture(texture, pixels)
    }

    pub fn update_texture_rec(&self, texture: Texture2D, rec: Rectangle, pixels: Vec<u8>) {
        Self::__update_texture_rec(texture, rec, pixels)
    }

    // Texture configuration methods

    pub fn gen_texture_mipmaps(&self, texture: &mut Texture2D) {
        Self::__gen_texture_mipmaps(texture)
    }

    pub fn set_texture_filter(&self, texture: Texture2D, filter: TextureFilter) {
        Self::__set_texture_filter(texture, filter)
    }

    pub fn set_texture_wrap(&self, texture: Texture2D, wrap: TextureWrap) {
        Self::__set_texture_wrap(texture, wrap)
    }

    // Texture drawing methods

    pub fn draw_texture(&self, texture: Texture2D, x: i32, y: i32, tint: Color) {
        Self::__draw_texture(texture, x, y, tint)
    }

    pub fn draw_texture_v(&self, texture: Texture2D, position: Vector2, tint: Color) {
        Self::__draw_texture_v(texture, position, tint)
    }

    pub fn draw_texture_ex(
        &self,
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        Self::__draw_texture_ex(texture, position, rotation, scale, tint)
    }

    pub fn draw_texture_rec(
        &self,
        texture: Texture2D,
        source: Rectangle,
        position: Vector2,
        tint: Color,
    ) {
        Self::__draw_texture_rec(texture, source, position, tint)
    }

    pub fn draw_texture_pro(
        &self,
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        Self::__draw_texture_pro(texture, source, dest, origin, rotation, tint)
    }

    pub fn draw_texture_n_patch(
        &self,
        texture: Texture2D,
        info: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        Self::__draw_texture_n_patch(texture, info, dest, origin, rotation, tint)
    }

    // Color/pixel related methods

    pub fn fade(&self, color: Color, alpha: f32) -> Color {
        Self::__fade(color, alpha)
    }

    pub fn color_to_int(&self, color: Color) -> i32 {
        Self::__color_to_int(color)
    }

    pub fn color_normalize(&self, color: Color) -> Vector4 {
        Self::__color_normalize(color)
    }

    pub fn color_from_normalized(&self, normalized: Vector4) -> Color {
        Self::__color_from_normalized(normalized)
    }

    pub fn color_to_hsv(&self, color: Color) -> Vector3 {
        Self::__color_to_hsv(color)
    }

    pub fn color_from_hsv(&self, hue: f32, saturation: f32, value: f32) -> Color {
        Self::__color_from_hsv(hue, saturation, value)
    }

    pub fn color_tint(&self, color: Color, tint: Color) -> Color {
        Self::__color_tint(color, tint)
    }

    pub fn color_brightness(&self, color: Color, factor: f32) -> Color {
        Self::__color_brightness(color, factor)
    }

    pub fn color_contrast(&self, color: Color, contrast: f32) -> Color {
        Self::__color_contrast(color, contrast)
    }

    pub fn color_alpha(&self, color: Color, alpha: f32) -> Color {
        Self::__color_alpha(color, alpha)
    }

    pub fn color_alpha_blend(&self, dst: Color, src: Color, tint: Color) -> Color {
        Self::__color_alpha_blend(dst, src, tint)
    }

    pub fn get_color(&self, hex_value: u32) -> Color {
        Self::__get_color(hex_value)
    }

    pub fn get_pixel_color(&self, ptr: Vec<u8>, format: PixelFormat) -> Color {
        Self::__get_pixel_color(ptr, format)
    }

    pub fn set_pixel_color(&self, ptr: &mut Vec<u8>, color: Color, format: PixelFormat) {
        Self::__set_pixel_color(ptr, color, format)
    }

    pub fn get_pixel_data_size(&self, width: i32, height: i32, format: impl Into<usize>) -> i32 {
        Self::__get_pixel_data_size(width, height, format)
    }
}
