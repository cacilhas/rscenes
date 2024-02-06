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
    /// Rotate camera
    fn rotate(&mut self, angle: f32, pivot: Vector3);
    /// Rotate to a specific direction
    fn set_y_axis_rotation(&mut self, angle: f32);
    /// Set elevation
    fn set_y_elevation(&mut self, angle: f32);
    /// Rotate over local coordinates
    fn rotate_local(&mut self, angle: f32, pivot: Vector3);
    /// Vector indicating the front direction
    fn front_vector(&self) -> Vector3;
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
        RcameraImpl::__update_camera(self, mode as usize)
    }

    fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        RcameraImpl::__update_camera_pro(self, movement, rotation, zoom)
    }

    fn rotate(&mut self, angle: f32, axis: Vector3) {
        self.target = self.position.add(self.front_vector().rotate(angle, axis));
    }

    fn set_y_axis_rotation(&mut self, angle: f32) {
        let mut front = self.front_vector();
        let leg = (front.x * front.x + front.z * front.z).sqrt();
        front.x = angle.cos() * leg;
        front.z = angle.sin() * leg;
        self.target = front.add(self.position);
    }

    fn set_y_elevation(&mut self, angle: f32) {
        let mut front = self.front_vector();
        let hypotenuse = front.length();
        front.y = angle.sin() * hypotenuse;
        self.target = front.add(self.position);
    }

    fn rotate_local(&mut self, angle: f32, pivot: Vector3) {
        self.rotate(
            angle,
            pivot.local_to_global(self.front_vector(), Vector3::UP),
        )
    }

    fn front_vector(&self) -> Vector3 {
        self.target.add(self.position.mul(-1.0))
    }
}
