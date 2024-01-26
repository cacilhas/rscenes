use raylib_ffi::{enums::*, *};
use std::{f32::consts::PI, ffi::c_uchar, fmt::Display, slice};

#[derive(Clone, Copy, Debug)]
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
}
