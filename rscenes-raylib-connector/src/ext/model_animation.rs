use crate::rmodels::Rmodels;
use raylib_ffi::*;
use std::fmt::Display;

pub trait ModelAnimationExt: Sized {
    fn load(filename: impl Display) -> Result<Vec<Self>, String>;
    fn unload(self);
    fn unload_animations(anims: Vec<Self>);
}

impl ModelAnimationExt for ModelAnimation {
    fn load(filename: impl Display) -> Result<Vec<Self>, String> {
        Rmodels::__load_model_animations(filename)
    }

    fn unload(self) {
        Rmodels::__unload_model_animation(self)
    }

    fn unload_animations(anims: Vec<Self>) {
        Rmodels::__unload_model_animations(anims)
    }
}
