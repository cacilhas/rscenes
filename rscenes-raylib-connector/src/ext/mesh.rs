use crate::rmodels::Rmodels;
use raylib_ffi::*;
use std::fmt::Display;

pub trait MeshExt {
    fn upload(&mut self, dynamic: bool);
    fn update_buffer(self, index: i32, data: Vec<u8>, offset: i32);
    fn unload(self);
    fn export(self, filename: impl Display) -> bool;
    fn get_bounding_box(self) -> BoundingBox;
    fn gen_tangents(&mut self);
}

impl MeshExt for Mesh {
    fn upload(&mut self, dynamic: bool) {
        Rmodels::__upload_mesh(self, dynamic)
    }

    fn update_buffer(self, index: i32, data: Vec<u8>, offset: i32) {
        Rmodels::__update_mesh_buffer(self, index, data, offset)
    }

    fn unload(self) {
        Rmodels::__unload_mesh(self)
    }

    fn export(self, filename: impl Display) -> bool {
        Rmodels::__export_mesh(self, filename)
    }

    fn get_bounding_box(self) -> BoundingBox {
        Rmodels::__get_mesh_bounding_box(self)
    }
    fn gen_tangents(&mut self) {
        Rmodels::__gen_mesh_tangents(self)
    }
}
