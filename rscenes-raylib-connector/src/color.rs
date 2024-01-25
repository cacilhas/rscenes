use raylib_ffi::colors;
pub use raylib_ffi::Color;

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
}
