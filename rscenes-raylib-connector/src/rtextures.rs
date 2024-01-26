use raylib_ffi::{enums::*, *};
use std::{ffi::c_uchar, fmt::Display, slice};

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
}
