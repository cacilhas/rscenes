use crate::rmodels::Rmodels;
use raylib_ffi::*;
use std::fmt::Display;

pub trait MaterialExt: Sized {
    fn load(filename: impl Display) -> Result<Vec<Self>, String>;
    fn default() -> Self;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn set_texture(&mut self, map_tpe: i32, texture: Texture2D);
}

impl MaterialExt for Material {
    fn load(filename: impl Display) -> Result<Vec<Self>, String> {
        Rmodels::__load_materials(filename)
    }

    fn default() -> Self {
        Rmodels::__load_material_default()
    }

    fn is_ready(self) -> bool {
        Rmodels::__is_material_ready(self)
    }

    fn unload(self) {
        Rmodels::__unload_material(self)
    }

    fn set_texture(&mut self, map_tpe: i32, texture: Texture2D) {
        Rmodels::__set_material_texture(self, map_tpe, texture)
    }
}
