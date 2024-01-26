use crate::ext::vector::*;
use raylib_ffi::{enums::*, *};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rcamera;

/// Crate only methods
impl Rcamera {
    pub(crate) fn __update_camera(camera: &mut Camera3D, mode: impl Into<usize>) {
        unsafe { UpdateCamera(camera, mode.into() as i32) }
    }

    pub(crate) fn __update_camera_pro(
        camera: &mut Camera3D,
        movement: Vector3,
        rotation: Vector3,
        zoom: f32,
    ) {
        unsafe { UpdateCameraPro(camera, movement, rotation, zoom) }
    }
}

/// Exported methods
impl Rcamera {
    pub fn update_camera(&self, camera: &mut Camera3D, mode: CameraMode) {
        Self::__update_camera(camera, mode)
    }

    pub fn update_camera_pro(
        &self,
        camera: &mut Camera3D,
        movement: Vector3,
        rotation: Vector3,
        zoom: f32,
    ) {
        Self::__update_camera_pro(camera, movement, rotation, zoom)
    }

    #[allow(non_snake_case)]
    pub fn default_camera_2D(&self) -> Camera2D {
        Camera2D {
            offset: Vector2::ZERO,
            target: Vector2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
        }
    }

    #[allow(non_snake_case)]
    pub fn default_camera_3D(&self) -> Camera3D {
        Camera3D {
            position: Vector3::BACK,
            target: Vector3::ZERO,
            up: Vector3::UP,
            fovy: 100.0,
            projection: enums::CameraProjection::Perspective as i32,
        }
    }
}
