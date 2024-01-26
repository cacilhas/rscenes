use crate::rtextures::Rtextures;
use raylib_ffi::*;

pub trait TextureExt {
    fn is_ready(self) -> bool;
    fn unload(self);
    fn update_gpu(self, pixels: Vec<u8>);
    fn update_gpu_rec(self, rec: Rectangle, pixels: Vec<u8>);
}

impl TextureExt for Texture2D {
    fn is_ready(self) -> bool {
        Rtextures::__is_texture_ready(self)
    }

    fn unload(self) {
        Rtextures::__unload_texture(self)
    }

    fn update_gpu(self, pixels: Vec<u8>) {
        Rtextures::__update_texture(self, pixels)
    }

    fn update_gpu_rec(self, rec: Rectangle, pixels: Vec<u8>) {
        Rtextures::__update_texture_rec(self, rec, pixels)
    }
}

pub trait RenderTextureExt {
    fn is_ready(self) -> bool;
    fn unload(self);
}

impl RenderTextureExt for RenderTexture2D {
    fn is_ready(self) -> bool {
        Rtextures::__is_render_texture_ready(self)
    }

    fn unload(self) {
        Rtextures::__unload_render_texture(self)
    }
}
