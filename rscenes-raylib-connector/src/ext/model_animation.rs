use crate::rmodels::RmodelsImpl;
use raylib_ffi::*;
use std::fmt::Display;

pub trait ModelAnimationExt: Sized {
    /// Load model animations from file
    fn load(filename: impl Display) -> Result<Vec<Self>, String>;
    /// Unload animation data
    fn unload(self);
    /// Unload animation array data
    fn unload_animations(anims: &mut [Self]);
}

impl ModelAnimationExt for ModelAnimation {
    fn load(filename: impl Display) -> Result<Vec<Self>, String> {
        RmodelsImpl::__load_model_animations(filename)
    }

    fn unload(self) {
        RmodelsImpl::__unload_model_animation(self)
    }

    fn unload_animations(anims: &mut [Self]) {
        RmodelsImpl::__unload_model_animations(anims)
    }
}
