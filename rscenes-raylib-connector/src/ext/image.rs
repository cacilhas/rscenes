use crate::rtextures::Rtextures;
use raylib_ffi::{enums::PixelFormat, *};
use std::fmt::Display;

pub trait ImageExt: Sized {
    /// Load image from file into CPU memory (RAM)
    fn load(filename: impl Display) -> Result<Self, String>;
    /// Load image from RAW file data
    fn load_raw(
        filename: impl Display,
        width: i32,
        height: i32,
        format: impl Into<usize>,
        header_size: i32,
    ) -> Result<Self, String>;
    /// Load image from SVG file data or string with specified size
    fn load_svg(filename_or_string: impl Display, width: i32, height: i32) -> Result<Self, String>;
    /// Load image sequence from file (frames appended to image.data)
    fn load_anim(filename: impl Display) -> Result<(Self, i32), String>;
    /// Load image from memory buffer, fileType refers to extension: i.e. '.png'
    fn load_from_memory(tpe: ImageType, data: &mut Vec<u8>) -> Result<Self, String>;
    /// Load image from GPU texture data
    fn load_from_texture(texture: Texture2D) -> Self;
    /// Load image from screen buffer and (screenshot)
    fn load_from_screen() -> Self;

    /// Generate image: plain color
    fn gen_color(width: i32, height: i32, color: Color) -> Self;
    /// Generate image: linear gradient, direction in radians, 0=Vertical gradient
    fn gen_gradient_linear(width: i32, height: i32, angle: f32, start: Color, end: Color) -> Self;
    /// Generate image: radial gradient
    fn gen_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self;
    // Generate image: square gradient
    fn gen_gradient_square(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self;
    /// Generate image: checked
    fn gen_checked(
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: Color,
        col2: Color,
    ) -> Self;
    /// Generate image: white noise
    fn gen_white_noise(width: i32, height: i32, factor: f32) -> Self;
    /// Generate image: perlin noise
    fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Self;
    /// Generate image: cellular algorithm, bigger tileSize means bigger cells
    fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Self;
    /// Generate image: grayscale image from text data
    fn gen_text(width: i32, height: i32, text: impl Display) -> Self;
    /// Create an image from text (custom sprite font)
    fn gen_text_ex(
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Self;

    /// Check whether an image is ready
    fn is_ready(self) -> bool;
    /// Unload image from CPU memory (RAM)
    fn unload(self);
    /// Export image data to file, returns true on success
    fn export(self, filename: impl Display) -> bool;
    /// Export image to memory buffer
    fn export_to_memory(self, tpe: ImageType) -> Result<Vec<u8>, String>;
    /// Export image as code file defining an array of bytes, returns true on success
    fn export_as_code(self, filename: impl Display) -> bool;

    /// Create an image duplicate (useful for transformations)
    fn copy(self) -> Self;
    /// Create an image from another image piece
    fn copy_rec(self, rec: Rectangle) -> Self;
    /// Convert image data to desired format
    fn format(&mut self, format: PixelFormat) -> &mut Self;
    /// Convert image to POT (power-of-two)
    fn to_pot(&mut self, fill: Color) -> &mut Self;
    /// Crop an image to a defined rectangle
    fn crop(&mut self, crop: Rectangle) -> &mut Self;
    /// Crop image depending on alpha value
    fn alpha_crop(&mut self, threshold: f32) -> &mut Self;
    /// Clear alpha channel to desired color
    fn alpha_clear(&mut self, color: Color, threshold: f32) -> &mut Self;
    /// Apply alpha mask to image
    fn alpha_mask(&mut self, alpha_mask: Image) -> &mut Self;
    /// Premultiply alpha channel
    fn alpha_premultiply(&mut self) -> &mut Self;
    /// Apply Gaussian blur using a box blur approximation
    fn blur_gaussian(&mut self, blur_size: i32) -> &mut Self;
    /// Resize image (Bicubic scaling algorithm)
    fn resize(&mut self, width: i32, height: i32) -> &mut Self;
    /// Resize image (Nearest-Neighbor scaling algorithm)
    fn resize_nn(&mut self, width: i32, height: i32) -> &mut Self;
    // Resize canvas and fill with color
    fn resize_canvas(
        &mut self,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        fill: Color,
    ) -> &mut Self;
    /// Compute all mipmap levels for a provided image
    fn compute_mipmaps(&mut self) -> &mut Self;
    /// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
    fn dither(&mut self, r: i32, g: i32, b: i32, a: i32) -> &mut Self;
    /// Flip image vertically
    fn flip(&mut self) -> &mut Self;
    /// Flip image horizontally
    fn flop(&mut self) -> &mut Self;
    // Rotate image by input angle in radians
    fn rotate(&mut self, angle: f32) -> &mut Self;
    /// Modify image color: tint
    fn color_tint(&mut self, tint: Color) -> &mut Self;
    /// Modify image color: invert
    fn color_invert(&mut self) -> &mut Self;
    /// Modify image color: grayscale
    fn color_grayscale(&mut self) -> &mut Self;
    /// Modify image color: contrast (-100 to 100)
    fn color_constrast(&mut self, contrast: f32) -> &mut Self;
    /// Modify image color: brightness (-255 to 255)
    fn color_brightness(&mut self, brightness: i32) -> &mut Self;
    /// Modify image color: replace color
    fn color_replace(&mut self, color: Color, replace: Color) -> &mut Self;
    /// Load colors palette from image as a Color array (RGBA - 32bit)
    fn load_palette(self, max_size: usize) -> Result<Vec<Color>, String>;
    /// Get image alpha border rectangle
    fn get_alpha_border(self, threshold: f32) -> Rectangle;
    /// Get image pixel color at (x, y) position
    fn get_color(self, x: i32, y: i32) -> Color;

    /// Clear image background with given color
    fn clear_background(&mut self, color: Color) -> &mut Self;
    /// Draw pixel within an image
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) -> &mut Self;
    /// Draw pixel within an image (Vector version)
    fn draw_pixel_v(&mut self, position: Vector2, color: Color) -> &mut Self;
    /// Draw line within an image
    fn draw_line(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Color,
    ) -> &mut Self;
    /// Draw line within an image (Vector version)
    fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) -> &mut Self;
    /// Draw a filled circle within an image
    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color)
        -> &mut Self;
    /// Draw a filled circle within an image (Vector version)
    fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) -> &mut Self;
    /// Draw circle outline within an image
    fn draw_circle_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Color,
    ) -> &mut Self;
    /// Draw circle outline within an image (Vector version)
    fn draw_circle_lines_v(&mut self, center: Vector2, radius: i32, color: Color) -> &mut Self;
    /// Draw rectangle within an image
    fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) -> &mut Self;
    /// Draw rectangle within an image (Vector version)
    fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) -> &mut Self;
    /// Draw rectangle within an image
    fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) -> &mut Self;
    /// Draw rectangle lines within an image
    fn draw_rectangle_lines(&mut self, rec: Rectangle, thick: i32, color: Color) -> &mut Self;
    /// Draw a source image within a destination image (tint applied to source)
    fn draw_image(
        &mut self,
        src: Self,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: Color,
    ) -> &mut Self;
    /// Draw text (using default font) within an image (destination)
    fn draw_text(
        &mut self,
        text: impl Display,
        x: i32,
        y: i32,
        font_size: i32,
        color: Color,
    ) -> &mut Self;
    /// Draw text (custom sprite font) within an image (destination)
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
    fn load(filename: impl Display) -> Result<Self, String> {
        Rtextures::__load_image(filename)
    }

    fn load_raw(
        filename: impl Display,
        width: i32,
        height: i32,
        format: impl Into<usize>,
        header_size: i32,
    ) -> Result<Self, String> {
        Rtextures::__load_image_raw(filename, width, height, format, header_size)
    }

    fn load_svg(filename_or_string: impl Display, width: i32, height: i32) -> Result<Self, String> {
        Rtextures::__load_image_svg(filename_or_string, width, height)
    }

    fn load_anim(filename: impl Display) -> Result<(Self, i32), String> {
        Rtextures::__load_image_anim(filename)
    }

    fn load_from_screen() -> Self {
        Rtextures::__load_image_from_screen()
    }

    fn load_from_texture(texture: Texture2D) -> Self {
        Rtextures::__load_image_from_texture(texture)
    }

    fn load_from_memory(tpe: ImageType, data: &mut Vec<u8>) -> Result<Self, String> {
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

    fn gen_text_ex(
        font: Font,
        text: impl Display,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Self {
        Rtextures::__image_text_ex(font, text, font_size, spacing, tint)
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

    fn export_to_memory(self, tpe: ImageType) -> Result<Vec<u8>, String> {
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

    fn load_palette(self, max_size: usize) -> Result<Vec<Color>, String> {
        Rtextures::__load_image_pallete(self, max_size as i32)
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

#[derive(Clone, Copy, Debug)]
pub enum ImageType {
    Bitmap,
    Heif,
    Jpeg,
    Pixmap,
    Png,
    Pnm,
    Raw,
    Svg,
    Tiff,
    WebP,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::Bitmap => f.write_str(".bmp"),
            ImageType::Heif => f.write_str(".heif"),
            ImageType::Jpeg => f.write_str(".jpg"),
            ImageType::Pixmap => f.write_str(".xpm"),
            ImageType::Png => f.write_str(".png"),
            ImageType::Pnm => f.write_str(".pnm"),
            ImageType::Raw => f.write_str(".raw"),
            ImageType::Svg => f.write_str(".svg"),
            ImageType::Tiff => f.write_str(".tiff"),
            ImageType::WebP => f.write_str(".webp"),
        }
    }
}
