use raylib_ffi::{enums::CameraMode, *};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RcameraImpl;

/// Crate only methods
impl RcameraImpl {
    pub fn __update_camera(camera: &mut Camera3D, mode: impl Into<usize>) {
        unsafe { UpdateCamera(camera, mode.into() as i32) }
    }

    pub fn __update_camera_pro(
        camera: &mut Camera3D,
        movement: Vector3,
        rotation: Vector3,
        zoom: f32,
    ) {
        unsafe { UpdateCameraPro(camera, movement, rotation, zoom) }
    }
}

/// Exported methods
pub trait Rcamera: Debug {
    /// Update camera position for selected mode
    fn update_camera(&self, camera: &mut Camera3D, mode: CameraMode) {
        RcameraImpl::__update_camera(camera, mode as usize)
    }

    /// Update camera movement/rotation
    fn update_camera_pro(
        &self,
        camera: &mut Camera3D,
        movement: Vector3,
        rotation: Vector3,
        zoom: f32,
    ) {
        RcameraImpl::__update_camera_pro(camera, movement, rotation, zoom)
    }
}
