use std::fmt::Display;

use crate::rtextures::RtexturesImpl;
use raylib_ffi::{enums::*, *};

pub trait TextureExt: Sized {
    /// Load texture from file into GPU memory (VRAM)
    fn load(filename: impl Display) -> Result<Self, String>;
    /// Load texture from image data
    fn load_from_image(image: Image) -> Result<Self, String>;

    /// Check whether a texture is ready
    fn is_ready(self) -> bool;
    /// Unload texture from GPU memory (VRAM)
    fn unload(self);
    /// Update GPU texture with new data
    fn update_gpu(self, pixels: &[u8]) -> Self;
    /// Update GPU texture rectangle with new data
    fn update_gpu_rec(self, rec: Rectangle, pixels: &[u8]) -> Self;
    /// Generate GPU mipmaps for a texture
    fn gen_mipmaps(&mut self) -> &mut Self;
    /// Set texture scaling filter mode
    fn set_filter(self, filter: TextureFilter) -> Self;
    /// Set texture wrapping mode
    fn set_wrap(self, wrap: TextureWrap) -> Self;
}

impl TextureExt for Texture2D {
    fn load(filename: impl Display) -> Result<Self, String> {
        RtexturesImpl::__load_texture(filename)
    }

    fn load_from_image(image: Image) -> Result<Self, String> {
        RtexturesImpl::__load_texture_from_image(image)
    }

    fn is_ready(self) -> bool {
        RtexturesImpl::__is_texture_ready(self)
    }

    fn unload(self) {
        RtexturesImpl::__unload_texture(self)
    }

    fn update_gpu(self, pixels: &[u8]) -> Self {
        RtexturesImpl::__update_texture(self, pixels);
        self
    }

    fn update_gpu_rec(self, rec: Rectangle, pixels: &[u8]) -> Self {
        RtexturesImpl::__update_texture_rec(self, rec, pixels);
        self
    }

    fn gen_mipmaps(&mut self) -> &mut Self {
        RtexturesImpl::__gen_texture_mipmaps(self);
        self
    }

    fn set_filter(self, filter: TextureFilter) -> Self {
        RtexturesImpl::__set_texture_filter(self, filter);
        self
    }

    fn set_wrap(self, wrap: TextureWrap) -> Self {
        RtexturesImpl::__set_texture_wrap(self, wrap);
        self
    }
}

pub trait TextureCubemapExt: Sized {
    fn load(image: Image, layout: CubemapLayout) -> Result<Self, String>;
}

impl TextureCubemapExt for TextureCubemap {
    fn load(image: Image, layout: CubemapLayout) -> Result<Self, String> {
        RtexturesImpl::__load_texture_cubemap(image, layout)
    }
}

pub trait RenderTextureExt {
    fn load(width: i32, height: i32) -> Self;

    fn is_ready(self) -> bool;
    fn unload(self);
}

impl RenderTextureExt for RenderTexture2D {
    fn load(width: i32, height: i32) -> Self {
        RtexturesImpl::__load_render_texture(width, height)
    }

    fn is_ready(self) -> bool {
        RtexturesImpl::__is_render_texture_ready(self)
    }

    fn unload(self) {
        RtexturesImpl::__unload_render_texture(self)
    }
}
