use crate::rtextures::Rtextures;
use raylib_ffi::{enums::PixelFormat, *};
use std::fmt::Display;

pub trait ImageExt {
    fn load(filename: impl Display) -> Self;
    fn load_raw(
        filename: impl Display,
        width: i32,
        height: i32,
        format: impl Into<usize>,
        header_size: i32,
    ) -> Self;
    fn load_svg(filename_or_string: impl Display, width: i32, height: i32) -> Self;
    fn load_anim(filename: impl Display) -> (Self, i32)
    where
        Self: Sized;
    fn load_from_memory(tpe: impl Display, data: &mut Vec<u8>) -> Self;
    fn load_from_texture(texture: Texture2D) -> Self;
    fn load_from_screen() -> Self;

    fn gen_color(width: i32, height: i32, color: Color) -> Self;
    fn gen_gradient_linear(width: i32, height: i32, angle: f32, start: Color, end: Color) -> Self;
    fn gen_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self;
    fn gen_gradient_square(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self;
    fn gen_checked(
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: Color,
        col2: Color,
    ) -> Self;
    fn gen_white_noise(width: i32, height: i32, factor: f32) -> Self;
    fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Self;
    fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Self;
    fn gen_text(width: i32, height: i32, text: impl Display) -> Self;
    fn gen_text_ex(text: impl Display, font_size: i32, color: Color) -> Self;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn export(self, filename: impl Display) -> bool;
    fn export_to_memory(self, tpe: impl Display) -> Vec<u8>;
    fn export_as_code(self, filename: impl Display) -> bool;

    fn copy(self) -> Self;
    fn copy_rec(self, rec: Rectangle) -> Self;
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
    fn unload_palette(self, palette: &mut Vec<Color>);
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
    fn load(filename: impl Display) -> Self {
        Rtextures::__load_image(filename)
    }

    fn load_raw(
        filename: impl Display,
        width: i32,
        height: i32,
        format: impl Into<usize>,
        header_size: i32,
    ) -> Self {
        Rtextures::__load_image_raw(filename, width, height, format, header_size)
    }

    fn load_svg(filename_or_string: impl Display, width: i32, height: i32) -> Self {
        Rtextures::__load_image_svg(filename_or_string, width, height)
    }

    fn load_anim(filename: impl Display) -> (Self, i32) {
        Rtextures::__load_image_anim(filename)
    }

    fn load_from_screen() -> Self {
        Rtextures::__load_image_from_screen()
    }

    fn load_from_texture(texture: Texture2D) -> Self {
        Rtextures::__load_image_from_texture(texture)
    }

    fn load_from_memory(tpe: impl Display, data: &mut Vec<u8>) -> Self {
        Rtextures::__load_image_from_memory(tpe, data)
    }

    fn gen_white_noise(width: i32, height: i32, factor: f32) -> Self {
        Rtextures::__gen_image_white_noise(width, height, factor)
    }

    fn gen_text(width: i32, height: i32, text: impl Display) -> Self {
        Rtextures::__gen_image_text(width, height, text)
    }

    fn gen_color(width: i32, height: i32, color: Color) -> Self {
        Rtextures::__gen_image_color(width, height, color)
    }

    fn gen_checked(
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: Color,
        col2: Color,
    ) -> Self {
        Rtextures::__gen_image_checked(width, height, checks_x, checks_y, col1, col2)
    }

    fn gen_text_ex(text: impl Display, font_size: i32, color: Color) -> Self {
        Rtextures::__image_text(text, font_size, color)
    }

    fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Self {
        Rtextures::__gen_image_cellular(width, height, tile_size)
    }

    fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Self {
        Rtextures::__gen_image_perlin_noise(width, height, offset_x, offset_y, scale)
    }

    fn gen_gradient_linear(width: i32, height: i32, angle: f32, start: Color, end: Color) -> Self {
        Rtextures::__gen_image_gradient_linear(width, height, angle, start, end)
    }

    fn gen_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self {
        Rtextures::__gen_image_gradient_radial(width, height, density, inner, outer)
    }

    fn gen_gradient_square(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self {
        Rtextures::__gen_image_gradient_square(width, height, density, inner, outer)
    }

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

    fn copy_rec(self, rec: Rectangle) -> Self {
        Rtextures::__image_from_image(self, rec)
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

    fn unload_palette(self, palette: &mut Vec<Color>) {
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
