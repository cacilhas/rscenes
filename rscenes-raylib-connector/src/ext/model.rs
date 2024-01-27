use crate::rmodels::Rmodels;
use raylib_ffi::*;
use std::fmt::Display;

pub trait ModelExt {
    fn load(filename: impl Display) -> Self;
    fn load_from_mesh(mesh: Mesh) -> Self;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn get_bounding_box(self) -> BoundingBox;
    fn set_mesh_material(&mut self, mesh_id: i32, material_id: i32);
}

impl ModelExt for Model {
    fn load(filename: impl Display) -> Self {
        Rmodels::__load_model(filename)
    }

    fn load_from_mesh(mesh: Mesh) -> Self {
        Rmodels::__load_model_from_mesh(mesh)
    }

    fn is_ready(self) -> bool {
        Rmodels::__is_model_ready(self)
    }

    fn unload(self) {
        Rmodels::__unload_model(self)
    }

    fn get_bounding_box(self) -> BoundingBox {
        Rmodels::__get_model_bounding_box(self)
    }

    fn set_mesh_material(&mut self, mesh_id: i32, material_id: i32) {
        Rmodels::__set_model_mesh_material(self, mesh_id, material_id)
    }
}
