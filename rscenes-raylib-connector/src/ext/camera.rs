use super::vector::{Vector2Ext, Vector3Ext};
use crate::rcamera::RcameraImpl;
use raylib_ffi::{enums::*, *};

pub trait Camera2DExt: Sized {
    /// Return a new camera with default values
    fn empty() -> Self;
}

impl Camera2DExt for Camera2D {
    fn empty() -> Self {
        Self {
            offset: Vector2::ZERO,
            target: Vector2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
        }
    }
}

pub trait Camera3DExt {
    /// Return a new camera with default values
    fn empty() -> Self;
    /// Update camera position for selected mode
    fn update(&mut self, mode: CameraMode);
    /// Update camera movement/rotation
    fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32);
}

impl Camera3DExt for Camera3D {
    fn empty() -> Self {
        Self {
            position: Vector3::BACK,
            target: Vector3::ZERO,
            up: Vector3::UP,
            fovy: 100.0,
            projection: enums::CameraProjection::Perspective as i32,
        }
    }

    fn update(&mut self, mode: CameraMode) {
        RcameraImpl::__update_camera(self, mode)
    }

    fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        RcameraImpl::__update_camera_pro(self, movement, rotation, zoom)
    }
}
