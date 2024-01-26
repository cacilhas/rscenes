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
    fn format(&mut self, format: PixelFormat) -> &mut Self;
    fn to_pot(&mut self, fill: Color) -> &mut Self;
    fn crop(&mut self, crop: Rectangle) -> &mut Self;
    fn alpha_crop(&mut self, threshold: f32) -> &mut Self;
    fn alpha_clear(&mut self, color: Color, threshold: f32) -> &mut Self;
    fn alpha_mask(&mut self, alpha_mask: Image) -> &mut Self;
    fn alpha_premultiply(&mut self) -> &mut Self;
    fn blur_gaussian(&mut self, blur_size: i32) -> &mut Self;
    fn resize(&mut self, width: i32, height: i32) -> &mut Self;
    fn resize_nn(&mut self, width: i32, height: i32) -> &mut Self;
    fn resize_canvas(
        &mut self,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) -> &mut Self;
    fn compute_mipmaps(&mut self) -> &mut Self;
    fn dither(&mut self, r: i32, g: i32, b: i32, a: i32) -> &mut Self;
    // Use ImageMagick notation for flip/flop
    fn flip(&mut self) -> &mut Self;
    fn flop(&mut self) -> &mut Self;
    fn rotate(&mut self, angle: f32) -> &mut Self;
    fn color_tint(&mut self, tint: Color) -> &mut Self;
    fn color_invert(&mut self) -> &mut Self;
    fn color_grayscale(&mut self) -> &mut Self;
    fn color_constrast(&mut self, contrast: f32) -> &mut Self;
    fn color_brightness(&mut self, brightness: i32) -> &mut Self;
    fn color_replace(&mut self, color: Color, replace: Color) -> &mut Self;
    fn load_palette(self, max_size: usize) -> Vec<Color>;
    fn unload_palette(self, palette: Vec<Color>);
    fn get_alpha_border(self, threshold: f32) -> Rectangle;
    fn get_color(self, x: i32, y: i32) -> Color;

    fn clear_background(&mut self, color: Color) -> &mut Self;
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) -> &mut Self;
    fn draw_pixel_v(&mut self, position: Vector2, color: Color) -> &mut Self;
    fn draw_line(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Color,
    ) -> &mut Self;
    fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) -> &mut Self;
    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color)
        -> &mut Self;
    fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) -> &mut Self;
    fn draw_circle_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) -> &mut Self;
    fn draw_circle_lines_v(&mut self, center: Vector2, radius: i32, color: Color) -> &mut Self;
    fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) -> &mut Self;
    fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) -> &mut Self;
    fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) -> &mut Self;
    fn draw_rectangle_lines(&mut self, rec: Rectangle, thick: i32, color: Color) -> &mut Self;
    fn draw_image(
        &mut self,
        src: Self,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: Color,
    ) -> &mut Self;
    fn draw_text(
        &mut self,
        text: impl Display,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) -> &mut Self;
    fn draw_text_ex(
        &mut self,
        font: Font,
        text: impl Display,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> &mut Self;
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

    fn format(&mut self, format: PixelFormat) -> &mut Self {
        Rtextures::__image_format(self, format);
        self
    }

    fn to_pot(&mut self, fill: Color) -> &mut Self {
        Rtextures::__image_to_pot(self, fill);
        self
    }

    fn crop(&mut self, crop: Rectangle) -> &mut Self {
        Rtextures::__image_crop(self, crop);
        self
    }

    fn alpha_crop(&mut self, threshold: f32) -> &mut Self {
        Rtextures::__image_alpha_crop(self, threshold);
        self
    }

    fn alpha_clear(&mut self, color: Color, threshold: f32) -> &mut Self {
        Rtextures::__image_alpha_clear(self, color, threshold);
        self
    }

    fn alpha_mask(&mut self, alpha_mask: Image) -> &mut Self {
        Rtextures::__image_alpha_mask(self, alpha_mask);
        self
    }

    fn alpha_premultiply(&mut self) -> &mut Self {
        Rtextures::__image_alpha_premultiply(self);
        self
    }

    fn blur_gaussian(&mut self, blur_size: i32) -> &mut Self {
        Rtextures::__image_blur_gaussian(self, blur_size);
        self
    }

    fn resize(&mut self, width: i32, height: i32) -> &mut Self {
        Rtextures::__image_resize(self, width, height);
        self
    }

    fn resize_nn(&mut self, width: i32, height: i32) -> &mut Self {
        Rtextures::__image_resize_nn(self, width, height);
        self
    }

    fn resize_canvas(
        &mut self,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) -> &mut Self {
        Rtextures::__image_resize_canvas(self, width, height, offset_x, offset_y, fill);
        self
    }

    fn compute_mipmaps(&mut self) -> &mut Self {
        Rtextures::__image_mipmaps(self);
        self
    }

    fn dither(&mut self, r: i32, g: i32, b: i32, a: i32) -> &mut Self {
        Rtextures::__image_dither(self, r, g, b, a);
        self
    }

    fn flip(&mut self) -> &mut Self {
        Rtextures::__image_flip_vertical(self);
        self
    }

    fn flop(&mut self) -> &mut Self {
        Rtextures::__image_flip_horizontal(self);
        self
    }

    fn rotate(&mut self, angle: f32) -> &mut Self {
        Rtextures::__image_rotate(self, angle);
        self
    }

    fn color_tint(&mut self, tint: Color) -> &mut Self {
        Rtextures::__image_color_tint(self, tint);
        self
    }

    fn color_invert(&mut self) -> &mut Self {
        Rtextures::__image_color_invert(self);
        self
    }

    fn color_grayscale(&mut self) -> &mut Self {
        Rtextures::__image_color_grayscale(self);
        self
    }

    fn color_constrast(&mut self, contrast: f32) -> &mut Self {
        Rtextures::__image_color_contrast(self, contrast);
        self
    }

    fn color_brightness(&mut self, brightness: i32) -> &mut Self {
        Rtextures::__image_color_brightness(self, brightness);
        self
    }

    fn color_replace(&mut self, color: Color, replace: Color) -> &mut Self {
        Rtextures::__image_color_replace(self, color, replace);
        self
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

    fn clear_background(&mut self, color: Color) -> &mut Self {
        Rtextures::__image_clear_background(self, color);
        self
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) -> &mut Self {
        Rtextures::__image_draw_pixel(self, x, y, color);
        self
    }

    fn draw_pixel_v(&mut self, position: Vector2, color: Color) -> &mut Self {
        Rtextures::__image_draw_pixel_v(self, position, color);
        self
    }

    fn draw_line(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Color,
    ) -> &mut Self {
        Rtextures::__image_draw_line(self, start_x, start_y, end_x, end_y, color);
        self
    }

    fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) -> &mut Self {
        Rtextures::__image_draw_line_v(self, start, end, color);
        self
    }

    fn draw_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) -> &mut Self {
        Rtextures::__image_draw_circle(self, center_x, center_y, radius, color);
        self
    }

    fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) -> &mut Self {
        Rtextures::__image_draw_circle_v(self, center, radius, color);
        self
    }

    fn draw_circle_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) -> &mut Self {
        Rtextures::__image_draw_circle_lines(self, center_x, center_y, radius, color);
        self
    }

    fn draw_circle_lines_v(&mut self, center: Vector2, radius: i32, color: Color) -> &mut Self {
        Rtextures::__image_draw_circle_lines_v(self, center, radius, color);
        self
    }

    fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) -> &mut Self {
        Rtextures::__image_draw_rectangle(self, x, y, width, height, color);
        self
    }

    fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) -> &mut Self {
        Rtextures::__image_draw_rectangle_v(self, position, size, color);
        self
    }

    fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) -> &mut Self {
        Rtextures::__image_draw_rectangle_rec(self, rec, color);
        self
    }

    fn draw_rectangle_lines(&mut self, rec: Rectangle, thick: i32, color: Color) -> &mut Self {
        Rtextures::__image_draw_rectangle_lines(self, rec, thick, color);
        self
    }

    fn draw_image(
        &mut self,
        src: Self,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: Color,
    ) -> &mut Self {
        Rtextures::__image_draw(self, src, src_rec, dst_rec, tint);
        self
    }

    fn draw_text(
        &mut self,
        text: impl Display,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) -> &mut Self {
        Rtextures::__image_draw_text(self, text, x, y, font_size, color);
        self
    }

    fn draw_text_ex(
        &mut self,
        font: Font,
        text: impl Display,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> &mut Self {
        Rtextures::__image_draw_text_ex(self, font, text, position, font_size, spacing, tint);
        self
    }
}
