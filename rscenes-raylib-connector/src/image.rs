use crate::rtextures::Rtextures;
use raylib_ffi::{enums::PixelFormat, *};
use std::fmt::Display;

pub trait ImageExt {
    fn is_ready(self) -> bool;
    fn unload(self);
    fn export(self, filename: impl Display) -> bool;
    fn export_to_memory(self, tpe: impl Display) -> Vec<u8>;
    fn export_as_code(self, filename: impl Display) -> bool;

    fn copy(self) -> Image;
    fn format(&mut self, format: PixelFormat);
    fn to_pot(&mut self, fill: Color);
    fn crop(&mut self, crop: Rectangle);
    fn alpha_crop(&mut self, threshold: f32);
    fn alpha_clear(&mut self, color: Color, threshold: f32);
    fn alpha_mask(&mut self, alpha_mask: Image);
    fn alpha_premultiply(&mut self);
    fn blur_gaussian(&mut self, blur_size: i32);
    fn resize(&mut self, width: i32, height: i32);
    fn resize_nn(&mut self, width: i32, height: i32);
    fn resize_canvas(&mut self, width: i32, height: i32, offset_x: i32, offset_y: i32, fill: Color);
    fn compute_mipmaps(&mut self);
    fn dither(&mut self, r: i32, g: i32, b: i32, a: i32);
    // Use ImageMagick notation for flip/flop
    fn flip(&mut self);
    fn flop(&mut self);
    fn rotate(&mut self, angle: f32);
    fn color_tint(&mut self, tint: Color);
    fn color_invert(&mut self);
    fn color_grayscale(&mut self);
    fn color_constrast(&mut self, contrast: f32);
    fn color_brightness(&mut self, brightness: i32);
    fn color_replace(&mut self, color: Color, replace: Color);
    fn load_palette(self, max_size: usize) -> Vec<Color>;
    fn unload_palette(self, palette: Vec<Color>);
    fn get_alpha_border(self, threshold: f32) -> Rectangle;
    fn get_color(self, x: i32, y: i32) -> Color;
}

impl ImageExt for Image {
    fn is_ready(self) -> bool {
        Rtextures::__is_image_ready(self)
    }

    fn unload(self) {
        Rtextures::__unload_image(self)
    }

    fn export(self, filename: impl Display) -> bool {
        Rtextures::__export_image(self, filename)
    }

    fn export_to_memory(self, tpe: impl Display) -> Vec<u8> {
        Rtextures::__export_image_to_memory(self, tpe)
    }

    fn export_as_code(self, filename: impl Display) -> bool {
        Rtextures::__export_image_as_code(self, filename)
    }

    fn copy(self) -> Image {
        Rtextures::__image_copy(self)
    }

    fn format(&mut self, format: PixelFormat) {
        Rtextures::__image_format(self, format)
    }

    fn to_pot(&mut self, fill: Color) {
        Rtextures::__image_to_pot(self, fill)
    }

    fn crop(&mut self, crop: Rectangle) {
        Rtextures::__image_crop(self, crop)
    }

    fn alpha_crop(&mut self, threshold: f32) {
        Rtextures::__image_alpha_crop(self, threshold)
    }

    fn alpha_clear(&mut self, color: Color, threshold: f32) {
        Rtextures::__image_alpha_clear(self, color, threshold)
    }

    fn alpha_mask(&mut self, alpha_mask: Image) {
        Rtextures::__image_alpha_mask(self, alpha_mask)
    }

    fn alpha_premultiply(&mut self) {
        Rtextures::__image_alpha_premultiply(self)
    }

    fn blur_gaussian(&mut self, blur_size: i32) {
        Rtextures::__image_blur_gaussian(self, blur_size)
    }

    fn resize(&mut self, width: i32, height: i32) {
        Rtextures::__image_resize(self, width, height)
    }

    fn resize_nn(&mut self, width: i32, height: i32) {
        Rtextures::__image_resize_nn(self, width, height)
    }

    fn resize_canvas(
        &mut self,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) {
        Rtextures::__image_resize_canvas(self, width, height, offset_x, offset_y, fill)
    }

    fn compute_mipmaps(&mut self) {
        Rtextures::__image_mipmaps(self)
    }

    fn dither(&mut self, r: i32, g: i32, b: i32, a: i32) {
        Rtextures::__image_dither(self, r, g, b, a)
    }

    fn flip(&mut self) {
        Rtextures::__image_flip_vertical(self)
    }

    fn flop(&mut self) {
        Rtextures::__image_flip_horizontal(self)
    }

    fn rotate(&mut self, angle: f32) {
        Rtextures::__image_rotate(self, angle)
    }

    fn color_tint(&mut self, tint: Color) {
        Rtextures::__image_color_tint(self, tint)
    }

    fn color_invert(&mut self) {
        Rtextures::__image_color_invert(self)
    }

    fn color_grayscale(&mut self) {
        Rtextures::__image_color_grayscale(self)
    }

    fn color_constrast(&mut self, contrast: f32) {
        Rtextures::__image_color_contrast(self, contrast)
    }

    fn color_brightness(&mut self, brightness: i32) {
        Rtextures::__image_color_brightness(self, brightness)
    }

    fn color_replace(&mut self, color: Color, replace: Color) {
        Rtextures::__image_color_replace(self, color, replace)
    }

    fn load_palette(self, max_size: usize) -> Vec<Color> {
        Rtextures::__load_image_pallete(self, max_size as i32)
    }

    fn unload_palette(self, palette: Vec<Color>) {
        Rtextures::__unload_image_palette(palette)
    }

    fn get_alpha_border(self, threshold: f32) -> Rectangle {
        Rtextures::__get_image_alpha_border(self, threshold)
    }

    fn get_color(self, x: i32, y: i32) -> Color {
        Rtextures::__get_image_color(self, x, y)
    }
}
