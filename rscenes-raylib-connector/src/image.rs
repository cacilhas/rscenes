use crate::rtextures::Rtextures;
use raylib_ffi::*;
use std::fmt::Display;

pub trait ImageExt {
    fn is_ready(self) -> bool;
    fn unload(self);
    fn export(self, filename: impl Display) -> bool;
    fn export_to_memory(self, tpe: impl Display) -> Vec<u8>;
    fn export_as_code(self, filename: impl Display) -> bool;
}

impl ImageExt for Image {
    fn is_ready(self) -> bool {
        Rtextures::__is_image_ready(self)
    }

    fn unload(self) {
        Rtextures::__unload_image(self)
    }

    fn export(self, filename: impl Display) -> bool {
        Rtextures::__export_image(self, filename)
    }

    fn export_to_memory(self, tpe: impl Display) -> Vec<u8> {
        Rtextures::__export_image_to_memory(self, tpe)
    }

    fn export_as_code(self, filename: impl Display) -> bool {
        Rtextures::__export_image_as_code(self, filename)
    }
}
