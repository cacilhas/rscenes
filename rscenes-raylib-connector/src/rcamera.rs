use raylib_ffi::{enums::CameraMode, *};

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
    /// Update camera position for selected mode
    pub fn update_camera(&self, camera: &mut Camera3D, mode: CameraMode) {
        Self::__update_camera(camera, mode)
    }

    /// Update camera movement/rotation
    pub fn update_camera_pro(
        &self,
        camera: &mut Camera3D,
        movement: Vector3,
        rotation: Vector3,
        zoom: f32,
    ) {
        Self::__update_camera_pro(camera, movement, rotation, zoom)
    }
}
