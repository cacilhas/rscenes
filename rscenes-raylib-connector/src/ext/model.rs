use crate::rmodels::Rmodels;
use raylib_ffi::*;
use std::fmt::Display;

pub trait ModelExt {
    fn load(filename: impl Display) -> Self;
    fn load_from_mesh(mesh: Mesh) -> Self;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn get_bounding_box(self) -> BoundingBox;
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
}
