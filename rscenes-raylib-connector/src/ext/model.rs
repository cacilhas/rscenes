use crate::rmodels::Rmodels;
use raylib_ffi::*;

pub trait ModelExt {
    fn is_ready(self) -> bool;
    fn unload(self);
    fn get_bounding_box(self) -> BoundingBox;
}

impl ModelExt for Model {
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
