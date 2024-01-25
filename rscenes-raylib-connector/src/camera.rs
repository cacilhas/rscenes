use crate::rcamera::Rcamera;
use raylib_ffi::{enums::CameraMode, Camera3D, Vector3};

pub trait Camera3DExt {
    fn update(&mut self, mode: CameraMode);
    fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32);
}

impl Camera3DExt for Camera3D {
    fn update(&mut self, mode: CameraMode) {
        Rcamera::__update_camera(self, mode)
    }

    fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        Rcamera::__update_camera_pro(self, movement, rotation, zoom)
    }
}
