use crate::rtextures::RtexturesImpl;
use raylib_ffi::{colors, Color, Vector3, Vector4};

pub trait ColorExt: Sized {
    // Default colours
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

    // Additional colours
    const INDIANRED: Self;
    const LIGHTCORAL: Self;
    const SALMON: Self;
    const DARKSALMON: Self;
    const LIGHTSALMON: Self;
    const CRIMSON: Self;
    const FIREBRICK: Self;
    const DARKRED: Self;
    const LIGHTPINK: Self;
    const HOTPINK: Self;
    const DEEPPINK: Self;
    const MEDIUMVIOLETRED: Self;
    const PALEVIOLETRED: Self;
    const CORAL: Self;
    const TOMATO: Self;
    const ORANGERED: Self;
    const DARKORANGE: Self;
    const LIGHTYELLOW: Self;
    const LEMONCHIFFON: Self;
    const LIGHTGOLDENRODYELLOW: Self;
    const PAPAYAWHIP: Self;
    const MOCCASIN: Self;
    const PEACHPUFF: Self;
    const PALEGOLDENROD: Self;
    const KHAKI: Self;
    const DARKKHAKI: Self;
    const LAVENDER: Self;
    const THISTLE: Self;
    const PLUM: Self;
    const ORCHID: Self;
    const FUCHSIA: Self;
    const MEDIUMORCHID: Self;
    const MEDIUMPURPLE: Self;
    const REBECCAPURPLE: Self;
    const BLUEVIOLET: Self;
    const DARKVIOLET: Self;
    const DARKORCHID: Self;
    const DARKMAGENTA: Self;
    const INDIGO: Self;
    const SLATEBLUE: Self;
    const DARKSLATEBLUE: Self;
    const MEDIUMSLATEBLUE: Self;
    const GREENYELLOW: Self;
    const CHARTREUSE: Self;
    const LAWNGREEN: Self;
    const LIMEGREEN: Self;
    const PALEGREEN: Self;
    const LIGHTGREEN: Self;
    const MEDIUMSPRINGGREEN: Self;
    const SPRINGGREEN: Self;
    const MEDIUMSEAGREEN: Self;
    const SEAGREEN: Self;
    const FORESTGREEN: Self;
    const YELLOWGREEN: Self;
    const OLIVEDRAB: Self;
    const OLIVE: Self;
    const DARKOLIVEGREEN: Self;
    const MEDIUMAQUAMARINE: Self;
    const DARKSEAGREEN: Self;
    const LIGHTSEAGREEN: Self;
    const DARKCYAN: Self;
    const TEAL: Self;
    const AQUA: Self;
    const CYAN: Self;
    const LIGHTCYAN: Self;
    const PALETURQUOISE: Self;
    const AQUAMARINE: Self;
    const TURQUOISE: Self;
    const MEDIUMTURQUOISE: Self;
    const DARKTURQUOISE: Self;
    const CADETBLUE: Self;
    const STEELBLUE: Self;
    const LIGHTSTEELBLUE: Self;
    const POWDERBLUE: Self;
    const LIGHTBLUE: Self;
    const LIGHTSKYBLUE: Self;
    const DEEPSKYBLUE: Self;
    const DODGERBLUE: Self;
    const CORNFLOWERBLUE: Self;
    const ROYALBLUE: Self;
    const MEDIUMBLUE: Self;
    const NAVY: Self;
    const MIDNIGHTBLUE: Self;
    const CORNSILK: Self;
    const BLANCHEDALMOND: Self;
    const BISQUE: Self;
    const NAVAJOWHITE: Self;
    const WHEAT: Self;
    const BURLYWOOD: Self;
    const TAN: Self;
    const ROSYBROWN: Self;
    const SANDYBROWN: Self;
    const GOLDENROD: Self;
    const DARKGOLDENROD: Self;
    const PERU: Self;
    const CHOCOLATE: Self;
    const SADDLEBROWN: Self;
    const SIENNA: Self;
    const SNOW: Self;
    const HONEYDEW: Self;
    const MINTCREAM: Self;
    const AZURE: Self;
    const ALICEBLUE: Self;
    const GHOSTWHITE: Self;
    const WHITESMOKE: Self;
    const SEASHELL: Self;
    const OLDLACE: Self;
    const FLORALWHITE: Self;
    const IVORY: Self;
    const ANTIQUEWHITE: Self;
    const LINEN: Self;
    const LAVENDERBLUSH: Self;
    const MISTYROSE: Self;
    const GAINSBORO: Self;
    const SILVER: Self;
    const DIMGRAY: Self;
    const LIGHTSLATEGRAY: Self;
    const SLATEGRAY: Self;
    const DARKSLATEGRAY: Self;

    /// Get Color from normalized values [0..1]
    fn from_normalized(normalized: Vector4) -> Self;
    /// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
    fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self;
    /// Get Color structure from hexadecimal value
    fn from_hex_value(hex_value: u32) -> Self;

    /// Get color with alpha applied, alpha goes from 0.0 to 1.0
    fn fade(self, alpha: f32) -> Self;
    /// Get hexadecimal value for a Color
    fn to_int(self) -> i32;
    /// Get Color normalized as float [0..1]
    fn normalize(self) -> Vector4;
    /// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
    fn to_hsv(self) -> Vector3;
    /// Get color multiplied with another color
    fn tint(self, tint: Self) -> Self;
    /// Get color with brightness correction, brightness factor goes from -1.0 to 1.0
    fn brightness(self, factor: f32) -> Self;
    /// Get color with contrast correction, contrast values between -1.0 and 1.0
    fn contrast(self, contrast: f32) -> Self;
    /// Get color with alpha applied, alpha goes from 0.0 to 1.0
    fn alpha(self, alpha: f32) -> Self;
    /// Get src alpha-blended into dst color with tint
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

    const INDIANRED: Self = Self {
        r: 0xcd,
        g: 0x5c,
        b: 0x5c,
        a: 0xff,
    };
    const LIGHTCORAL: Self = Self {
        r: 0xf0,
        g: 0x80,
        b: 0x80,
        a: 0xff,
    };
    const SALMON: Self = Self {
        r: 0xfa,
        g: 0x80,
        b: 0x72,
        a: 0xff,
    };
    const DARKSALMON: Self = Self {
        r: 0xe9,
        g: 0x96,
        b: 0x7a,
        a: 0xff,
    };
    const LIGHTSALMON: Self = Self {
        r: 0xff,
        g: 0xa0,
        b: 0x7a,
        a: 0xff,
    };
    const CRIMSON: Self = Self {
        r: 0xdc,
        g: 0x14,
        b: 0x3c,
        a: 0xff,
    };
    const FIREBRICK: Self = Self {
        r: 0xb2,
        g: 0x22,
        b: 0x22,
        a: 0xff,
    };
    const DARKRED: Self = Self {
        r: 0x8b,
        g: 0x00,
        b: 0x00,
        a: 0xff,
    };
    const LIGHTPINK: Self = Self {
        r: 0xff,
        g: 0xb6,
        b: 0xc1,
        a: 0xff,
    };
    const HOTPINK: Self = Self {
        r: 0xff,
        g: 0x69,
        b: 0xb4,
        a: 0xff,
    };
    const DEEPPINK: Self = Self {
        r: 0xff,
        g: 0x14,
        b: 0x93,
        a: 0xff,
    };
    const MEDIUMVIOLETRED: Self = Self {
        r: 0xc7,
        g: 0x15,
        b: 0x85,
        a: 0xff,
    };
    const PALEVIOLETRED: Self = Self {
        r: 0xdb,
        g: 0x70,
        b: 0x93,
        a: 0xff,
    };
    const CORAL: Self = Self {
        r: 0xff,
        g: 0x7f,
        b: 0x50,
        a: 0xff,
    };
    const TOMATO: Self = Self {
        r: 0xff,
        g: 0x63,
        b: 0x47,
        a: 0xff,
    };
    const ORANGERED: Self = Self {
        r: 0xff,
        g: 0x45,
        b: 0x00,
        a: 0xff,
    };
    const DARKORANGE: Self = Self {
        r: 0xff,
        g: 0x8c,
        b: 0x00,
        a: 0xff,
    };
    const LIGHTYELLOW: Self = Self {
        r: 0xff,
        g: 0xff,
        b: 0xe0,
        a: 0xff,
    };
    const LEMONCHIFFON: Self = Self {
        r: 0xff,
        g: 0xfa,
        b: 0xcd,
        a: 0xff,
    };
    const LIGHTGOLDENRODYELLOW: Self = Self {
        r: 0xfa,
        g: 0xfa,
        b: 0xd2,
        a: 0xff,
    };
    const PAPAYAWHIP: Self = Self {
        r: 0xff,
        g: 0xef,
        b: 0xd5,
        a: 0xff,
    };
    const MOCCASIN: Self = Self {
        r: 0xff,
        g: 0xe4,
        b: 0xb5,
        a: 0xff,
    };
    const PEACHPUFF: Self = Self {
        r: 0xff,
        g: 0xda,
        b: 0xb9,
        a: 0xff,
    };
    const PALEGOLDENROD: Self = Self {
        r: 0xee,
        g: 0xe8,
        b: 0xaa,
        a: 0xff,
    };
    const KHAKI: Self = Self {
        r: 0xf0,
        g: 0xe6,
        b: 0x8c,
        a: 0xff,
    };
    const DARKKHAKI: Self = Self {
        r: 0xbd,
        g: 0xb7,
        b: 0x6b,
        a: 0xff,
    };
    const LAVENDER: Self = Self {
        r: 0xe6,
        g: 0xe6,
        b: 0xfa,
        a: 0xff,
    };
    const THISTLE: Self = Self {
        r: 0xd8,
        g: 0xbf,
        b: 0xd8,
        a: 0xff,
    };
    const PLUM: Self = Self {
        r: 0xdd,
        g: 0xa0,
        b: 0xdd,
        a: 0xff,
    };
    const ORCHID: Self = Self {
        r: 0xda,
        g: 0x70,
        b: 0xd6,
        a: 0xff,
    };
    const FUCHSIA: Self = Self {
        r: 0xff,
        g: 0x00,
        b: 0xff,
        a: 0xff,
    };
    const MEDIUMORCHID: Self = Self {
        r: 0xba,
        g: 0x55,
        b: 0xd3,
        a: 0xff,
    };
    const MEDIUMPURPLE: Self = Self {
        r: 0x93,
        g: 0x70,
        b: 0xdb,
        a: 0xff,
    };
    const REBECCAPURPLE: Self = Self {
        r: 0x66,
        g: 0x33,
        b: 0x99,
        a: 0xff,
    };
    const BLUEVIOLET: Self = Self {
        r: 0x8a,
        g: 0x2b,
        b: 0xe2,
        a: 0xff,
    };
    const DARKVIOLET: Self = Self {
        r: 0x94,
        g: 0x00,
        b: 0xd3,
        a: 0xff,
    };
    const DARKORCHID: Self = Self {
        r: 0x99,
        g: 0x32,
        b: 0xcc,
        a: 0xff,
    };
    const DARKMAGENTA: Self = Self {
        r: 0x8b,
        g: 0x00,
        b: 0x8b,
        a: 0xff,
    };
    const INDIGO: Self = Self {
        r: 0x4b,
        g: 0x00,
        b: 0x82,
        a: 0xff,
    };
    const SLATEBLUE: Self = Self {
        r: 0x6a,
        g: 0x5a,
        b: 0xcd,
        a: 0xff,
    };
    const DARKSLATEBLUE: Self = Self {
        r: 0x48,
        g: 0x3d,
        b: 0x8b,
        a: 0xff,
    };
    const MEDIUMSLATEBLUE: Self = Self {
        r: 0x7b,
        g: 0x68,
        b: 0xee,
        a: 0xff,
    };
    const GREENYELLOW: Self = Self {
        r: 0xad,
        g: 0xff,
        b: 0x2f,
        a: 0xff,
    };
    const CHARTREUSE: Self = Self {
        r: 0x7f,
        g: 0xff,
        b: 0x00,
        a: 0xff,
    };
    const LAWNGREEN: Self = Self {
        r: 0x7c,
        g: 0xfc,
        b: 0x00,
        a: 0xff,
    };
    const LIMEGREEN: Self = Self {
        r: 0x32,
        g: 0xcd,
        b: 0x32,
        a: 0xff,
    };
    const PALEGREEN: Self = Self {
        r: 0x98,
        g: 0xfb,
        b: 0x98,
        a: 0xff,
    };
    const LIGHTGREEN: Self = Self {
        r: 0x90,
        g: 0xee,
        b: 0x90,
        a: 0xff,
    };
    const MEDIUMSPRINGGREEN: Self = Self {
        r: 0x00,
        g: 0xfa,
        b: 0x9a,
        a: 0xff,
    };
    const SPRINGGREEN: Self = Self {
        r: 0x00,
        g: 0xff,
        b: 0x7f,
        a: 0xff,
    };
    const MEDIUMSEAGREEN: Self = Self {
        r: 0x3c,
        g: 0xb3,
        b: 0x71,
        a: 0xff,
    };
    const SEAGREEN: Self = Self {
        r: 0x2e,
        g: 0x8b,
        b: 0x57,
        a: 0xff,
    };
    const FORESTGREEN: Self = Self {
        r: 0x22,
        g: 0x8b,
        b: 0x22,
        a: 0xff,
    };
    const YELLOWGREEN: Self = Self {
        r: 0x9a,
        g: 0xcd,
        b: 0x32,
        a: 0xff,
    };
    const OLIVEDRAB: Self = Self {
        r: 0x6b,
        g: 0x8e,
        b: 0x23,
        a: 0xff,
    };
    const OLIVE: Self = Self {
        r: 0x80,
        g: 0x80,
        b: 0x00,
        a: 0xff,
    };
    const DARKOLIVEGREEN: Self = Self {
        r: 0x55,
        g: 0x6b,
        b: 0x2f,
        a: 0xff,
    };
    const MEDIUMAQUAMARINE: Self = Self {
        r: 0x66,
        g: 0xcd,
        b: 0xaa,
        a: 0xff,
    };
    const DARKSEAGREEN: Self = Self {
        r: 0x8f,
        g: 0xbc,
        b: 0x8b,
        a: 0xff,
    };
    const LIGHTSEAGREEN: Self = Self {
        r: 0x20,
        g: 0xb2,
        b: 0xaa,
        a: 0xff,
    };
    const DARKCYAN: Self = Self {
        r: 0x00,
        g: 0x8b,
        b: 0x8b,
        a: 0xff,
    };
    const TEAL: Self = Self {
        r: 0x00,
        g: 0x80,
        b: 0x80,
        a: 0xff,
    };
    const AQUA: Self = Self {
        r: 0x00,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    const CYAN: Self = Self {
        r: 0x00,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    const LIGHTCYAN: Self = Self {
        r: 0xe0,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    const PALETURQUOISE: Self = Self {
        r: 0xaf,
        g: 0xee,
        b: 0xee,
        a: 0xff,
    };
    const AQUAMARINE: Self = Self {
        r: 0x7f,
        g: 0xff,
        b: 0xd4,
        a: 0xff,
    };
    const TURQUOISE: Self = Self {
        r: 0x40,
        g: 0xe0,
        b: 0xd0,
        a: 0xff,
    };
    const MEDIUMTURQUOISE: Self = Self {
        r: 0x48,
        g: 0xd1,
        b: 0xcc,
        a: 0xff,
    };
    const DARKTURQUOISE: Self = Self {
        r: 0x00,
        g: 0xce,
        b: 0xd1,
        a: 0xff,
    };
    const CADETBLUE: Self = Self {
        r: 0x5f,
        g: 0x9e,
        b: 0xa0,
        a: 0xff,
    };
    const STEELBLUE: Self = Self {
        r: 0x46,
        g: 0x82,
        b: 0xb4,
        a: 0xff,
    };
    const LIGHTSTEELBLUE: Self = Self {
        r: 0xb0,
        g: 0xc4,
        b: 0xde,
        a: 0xff,
    };
    const POWDERBLUE: Self = Self {
        r: 0xb0,
        g: 0xe0,
        b: 0xe6,
        a: 0xff,
    };
    const LIGHTBLUE: Self = Self {
        r: 0xad,
        g: 0xd8,
        b: 0xe6,
        a: 0xff,
    };
    const LIGHTSKYBLUE: Self = Self {
        r: 0x87,
        g: 0xce,
        b: 0xfa,
        a: 0xff,
    };
    const DEEPSKYBLUE: Self = Self {
        r: 0x00,
        g: 0xbf,
        b: 0xff,
        a: 0xff,
    };
    const DODGERBLUE: Self = Self {
        r: 0x1e,
        g: 0x90,
        b: 0xff,
        a: 0xff,
    };
    const CORNFLOWERBLUE: Self = Self {
        r: 0x64,
        g: 0x95,
        b: 0xed,
        a: 0xff,
    };
    const ROYALBLUE: Self = Self {
        r: 0x41,
        g: 0x69,
        b: 0xe1,
        a: 0xff,
    };
    const MEDIUMBLUE: Self = Self {
        r: 0x00,
        g: 0x00,
        b: 0xcd,
        a: 0xff,
    };
    const NAVY: Self = Self {
        r: 0x00,
        g: 0x00,
        b: 0x80,
        a: 0xff,
    };
    const MIDNIGHTBLUE: Self = Self {
        r: 0x19,
        g: 0x19,
        b: 0x70,
        a: 0xff,
    };
    const CORNSILK: Self = Self {
        r: 0xff,
        g: 0xf8,
        b: 0xdc,
        a: 0xff,
    };
    const BLANCHEDALMOND: Self = Self {
        r: 0xff,
        g: 0xeb,
        b: 0xcd,
        a: 0xff,
    };
    const BISQUE: Self = Self {
        r: 0xff,
        g: 0xe4,
        b: 0xc4,
        a: 0xff,
    };
    const NAVAJOWHITE: Self = Self {
        r: 0xff,
        g: 0xde,
        b: 0xad,
        a: 0xff,
    };
    const WHEAT: Self = Self {
        r: 0xf5,
        g: 0xde,
        b: 0xb3,
        a: 0xff,
    };
    const BURLYWOOD: Self = Self {
        r: 0xde,
        g: 0xb8,
        b: 0x87,
        a: 0xff,
    };
    const TAN: Self = Self {
        r: 0xd2,
        g: 0xb4,
        b: 0x8c,
        a: 0xff,
    };
    const ROSYBROWN: Self = Self {
        r: 0xbc,
        g: 0x8f,
        b: 0x8f,
        a: 0xff,
    };
    const SANDYBROWN: Self = Self {
        r: 0xf4,
        g: 0xa4,
        b: 0x60,
        a: 0xff,
    };
    const GOLDENROD: Self = Self {
        r: 0xda,
        g: 0xa5,
        b: 0x20,
        a: 0xff,
    };
    const DARKGOLDENROD: Self = Self {
        r: 0xb8,
        g: 0x86,
        b: 0x0b,
        a: 0xff,
    };
    const PERU: Self = Self {
        r: 0xcd,
        g: 0x85,
        b: 0x3f,
        a: 0xff,
    };
    const CHOCOLATE: Self = Self {
        r: 0xd2,
        g: 0x69,
        b: 0x1e,
        a: 0xff,
    };
    const SADDLEBROWN: Self = Self {
        r: 0x8b,
        g: 0x45,
        b: 0x13,
        a: 0xff,
    };
    const SIENNA: Self = Self {
        r: 0xa0,
        g: 0x52,
        b: 0x2d,
        a: 0xff,
    };
    const SNOW: Self = Self {
        r: 0xff,
        g: 0xfa,
        b: 0xfa,
        a: 0xff,
    };
    const HONEYDEW: Self = Self {
        r: 0xf0,
        g: 0xff,
        b: 0xf0,
        a: 0xff,
    };
    const MINTCREAM: Self = Self {
        r: 0xf5,
        g: 0xff,
        b: 0xfa,
        a: 0xff,
    };
    const AZURE: Self = Self {
        r: 0xf0,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    const ALICEBLUE: Self = Self {
        r: 0xf0,
        g: 0xf8,
        b: 0xff,
        a: 0xff,
    };
    const GHOSTWHITE: Self = Self {
        r: 0xf8,
        g: 0xf8,
        b: 0xff,
        a: 0xff,
    };
    const WHITESMOKE: Self = Self {
        r: 0xf5,
        g: 0xf5,
        b: 0xf5,
        a: 0xff,
    };
    const SEASHELL: Self = Self {
        r: 0xff,
        g: 0xf5,
        b: 0xee,
        a: 0xff,
    };
    const OLDLACE: Self = Self {
        r: 0xfd,
        g: 0xf5,
        b: 0xe6,
        a: 0xff,
    };
    const FLORALWHITE: Self = Self {
        r: 0xff,
        g: 0xfa,
        b: 0xf0,
        a: 0xff,
    };
    const IVORY: Self = Self {
        r: 0xff,
        g: 0xff,
        b: 0xf0,
        a: 0xff,
    };
    const ANTIQUEWHITE: Self = Self {
        r: 0xfa,
        g: 0xeb,
        b: 0xd7,
        a: 0xff,
    };
    const LINEN: Self = Self {
        r: 0xfa,
        g: 0xf0,
        b: 0xe6,
        a: 0xff,
    };
    const LAVENDERBLUSH: Self = Self {
        r: 0xff,
        g: 0xf0,
        b: 0xf5,
        a: 0xff,
    };
    const MISTYROSE: Self = Self {
        r: 0xff,
        g: 0xe4,
        b: 0xe1,
        a: 0xff,
    };
    const GAINSBORO: Self = Self {
        r: 0xdc,
        g: 0xdc,
        b: 0xdc,
        a: 0xff,
    };
    const SILVER: Self = Self {
        r: 0xc0,
        g: 0xc0,
        b: 0xc0,
        a: 0xff,
    };
    const DIMGRAY: Self = Self {
        r: 0x69,
        g: 0x69,
        b: 0x69,
        a: 0xff,
    };
    const LIGHTSLATEGRAY: Self = Self {
        r: 0x77,
        g: 0x88,
        b: 0x99,
        a: 0xff,
    };
    const SLATEGRAY: Self = Self {
        r: 0x70,
        g: 0x80,
        b: 0x90,
        a: 0xff,
    };
    const DARKSLATEGRAY: Self = Self {
        r: 0x2f,
        g: 0x4f,
        b: 0x4f,
        a: 0xff,
    };

    fn from_normalized(normalized: Vector4) -> Self {
        RtexturesImpl::__color_from_normalized(normalized)
    }

    fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        RtexturesImpl::__color_from_hsv(hue, saturation, value)
    }

    fn from_hex_value(hex_value: u32) -> Self {
        RtexturesImpl::__get_color(hex_value)
    }

    fn fade(self, alpha: f32) -> Self {
        RtexturesImpl::__fade(self, alpha)
    }

    fn to_int(self) -> i32 {
        RtexturesImpl::__color_to_int(self)
    }

    fn normalize(self) -> Vector4 {
        RtexturesImpl::__color_normalize(self)
    }

    fn to_hsv(self) -> Vector3 {
        RtexturesImpl::__color_to_hsv(self)
    }

    fn tint(self, tint: Self) -> Self {
        RtexturesImpl::__color_tint(self, tint)
    }

    fn brightness(self, factor: f32) -> Self {
        RtexturesImpl::__color_brightness(self, factor)
    }

    fn contrast(self, contrast: f32) -> Self {
        RtexturesImpl::__color_contrast(self, contrast)
    }

    fn alpha(self, alpha: f32) -> Self {
        RtexturesImpl::__color_alpha(self, alpha)
    }

    fn alpha_blend(self, src: Self, tint: Self) -> Self {
        RtexturesImpl::__color_alpha_blend(self, src, tint)
    }
}
