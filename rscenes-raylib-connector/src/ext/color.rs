use crate::rtextures::Rtextures;
use raylib_ffi::{colors, Color, Vector3, Vector4};

pub trait ColorExt {
    const LIGHTGRAY: Self;
    const GRAY: Self;
    const DARKGRAY: Self;
    const YELLOW: Self;
    const GOLD: Self;
    const ORANGE: Self;
    const PINK: Self;
    const RED: Self;
    const MAROON: Self;
    const GREEN: Self;
    const LIME: Self;
    const DARKGREEN: Self;
    const SKYBLUE: Self;
    const BLUE: Self;
    const DARKBLUE: Self;
    const PURPLE: Self;
    const VIOLET: Self;
    const DARKPURPLE: Self;
    const BEIGE: Self;
    const BROWN: Self;
    const DARKBROWN: Self;
    const WHITE: Self;
    const BLACK: Self;
    const BLANK: Self;
    const MAGENTA: Self;
    const RAYWHITE: Self;

    fn fade(self, alpha: f32) -> Self;
    fn to_int(self) -> i32;
    fn normalize(self) -> Vector4;
    fn to_hsv(self) -> Vector3;
    fn tint(self, tint: Self) -> Self;
    fn brightness(self, factor: f32) -> Self;
    fn contrast(self, contrast: f32) -> Self;
    fn alpha(self, alpha: f32) -> Self;
    fn alpha_blend(self, src: Self, tint: Self) -> Self;
}

impl ColorExt for Color {
    const LIGHTGRAY: Self = colors::LIGHTGRAY;
    const GRAY: Self = colors::GRAY;
    const DARKGRAY: Self = colors::DARKGRAY;
    const YELLOW: Self = colors::YELLOW;
    const GOLD: Self = colors::GOLD;
    const ORANGE: Self = colors::ORANGE;
    const PINK: Self = colors::PINK;
    const RED: Self = colors::RED;
    const MAROON: Self = colors::MAROON;
    const GREEN: Self = colors::GREEN;
    const LIME: Self = colors::LIME;
    const DARKGREEN: Self = colors::DARKGREEN;
    const SKYBLUE: Self = colors::SKYBLUE;
    const BLUE: Self = colors::BLUE;
    const DARKBLUE: Self = colors::DARKBLUE;
    const PURPLE: Self = colors::PURPLE;
    const VIOLET: Self = colors::VIOLET;
    const DARKPURPLE: Self = colors::DARKPURPLE;
    const BEIGE: Self = colors::BEIGE;
    const BROWN: Self = colors::BROWN;
    const DARKBROWN: Self = colors::DARKBROWN;
    const WHITE: Self = colors::WHITE;
    const BLACK: Self = colors::BLACK;
    const BLANK: Self = colors::BLANK;
    const MAGENTA: Self = colors::MAGENTA;
    const RAYWHITE: Self = colors::RAYWHITE;

    fn fade(self, alpha: f32) -> Self {
        Rtextures::__fade(self, alpha)
    }

    fn to_int(self) -> i32 {
        Rtextures::__color_to_int(self)
    }

    fn normalize(self) -> Vector4 {
        Rtextures::__color_normalize(self)
    }

    fn to_hsv(self) -> Vector3 {
        Rtextures::__color_to_hsv(self)
    }

    fn tint(self, tint: Self) -> Self {
        Rtextures::__color_tint(self, tint)
    }

    fn brightness(self, factor: f32) -> Self {
        Rtextures::__color_brightness(self, factor)
    }

    fn contrast(self, contrast: f32) -> Self {
        Rtextures::__color_contrast(self, contrast)
    }

    fn alpha(self, alpha: f32) -> Self {
        Rtextures::__color_alpha(self, alpha)
    }

    fn alpha_blend(self, src: Self, tint: Self) -> Self {
        Rtextures::__color_alpha_blend(self, src, tint)
    }
}
