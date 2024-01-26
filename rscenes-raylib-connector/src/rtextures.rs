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
            let direction = (angle * PI / 180.0) as i32;
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
}
