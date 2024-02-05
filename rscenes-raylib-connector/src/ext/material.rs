use crate::rmodels::RmodelsImpl;
use raylib_ffi::*;
use std::fmt::Display;

pub trait MaterialExt: Sized {
    /// Load materials from model file
    fn load(filename: impl Display) -> Result<Vec<Self>, String>;
    /// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
    fn default() -> Self;

    /// check whether a material is ready
    fn is_ready(self) -> bool;
    /// Unload material from GPU memory (VRAM)
    fn unload(self);
    /// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
    fn set_texture(&mut self, map_tpe: i32, texture: Texture2D);
}

impl MaterialExt for Material {
    fn load(filename: impl Display) -> Result<Vec<Self>, String> {
        RmodelsImpl::__load_materials(filename)
    }

    fn default() -> Self {
        RmodelsImpl::__load_material_default()
    }

    fn is_ready(self) -> bool {
        RmodelsImpl::__is_material_ready(self)
    }

    fn unload(self) {
        RmodelsImpl::__unload_material(self)
    }

    fn set_texture(&mut self, map_tpe: i32, texture: Texture2D) {
        RmodelsImpl::__set_material_texture(self, map_tpe, texture)
    }
}
