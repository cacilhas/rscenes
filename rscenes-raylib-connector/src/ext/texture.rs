use std::fmt::Display;

use crate::rtextures::Rtextures;
use raylib_ffi::{enums::*, *};

pub trait TextureExt {
    fn load(filename: impl Display) -> Result<Self, String>
    where
        Self: Sized;
    fn load_from_image(image: Image) -> Result<Self, String>
    where
        Self: Sized;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn update_gpu(self, pixels: &Vec<u8>) -> Self;
    fn update_gpu_rec(self, rec: Rectangle, pixels: &Vec<u8>) -> Self;
    fn gen_mipmaps(&mut self) -> &mut Self;
    fn set_filter(self, filter: TextureFilter) -> Self;
    fn set_wrap(self, wrap: TextureWrap) -> Self;
}

impl TextureExt for Texture2D {
    fn load(filename: impl Display) -> Result<Self, String> {
        Rtextures::__load_texture(filename)
    }

    fn load_from_image(image: Image) -> Result<Self, String> {
        Rtextures::__load_texture_from_image(image)
    }

    fn is_ready(self) -> bool {
        Rtextures::__is_texture_ready(self)
    }

    fn unload(self) {
        Rtextures::__unload_texture(self)
    }

    fn update_gpu(self, pixels: &Vec<u8>) -> Self {
        Rtextures::__update_texture(self, pixels);
        self
    }

    fn update_gpu_rec(self, rec: Rectangle, pixels: &Vec<u8>) -> Self {
        Rtextures::__update_texture_rec(self, rec, pixels);
        self
    }

    fn gen_mipmaps(&mut self) -> &mut Self {
        Rtextures::__gen_texture_mipmaps(self);
        self
    }

    fn set_filter(self, filter: TextureFilter) -> Self {
        Rtextures::__set_texture_filter(self, filter);
        self
    }

    fn set_wrap(self, wrap: TextureWrap) -> Self {
        Rtextures::__set_texture_wrap(self, wrap);
        self
    }
}

pub trait TextureCubemapExt {
    fn load(image: Image, layout: CubemapLayout) -> Result<Self, String>
    where
        Self: Sized;
}

impl TextureCubemapExt for TextureCubemap {
    fn load(image: Image, layout: CubemapLayout) -> Result<Self, String> {
        Rtextures::__load_texture_cubemap(image, layout)
    }
}

pub trait RenderTextureExt {
    fn load(width: i32, height: i32) -> Self;

    fn is_ready(self) -> bool;
    fn unload(self);
}

impl RenderTextureExt for RenderTexture2D {
    fn load(width: i32, height: i32) -> Self {
        Rtextures::__load_render_texture(width, height)
    }

    fn is_ready(self) -> bool {
        Rtextures::__is_render_texture_ready(self)
    }

    fn unload(self) {
        Rtextures::__unload_render_texture(self)
    }
}
