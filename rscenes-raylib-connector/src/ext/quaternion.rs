use super::vector::Vector3Ext;
use raylib_ffi::{Quaternion, Vector3};
use std::f32::consts::TAU;

pub trait QuaternionExt: Sized {
    fn from_vector(vector: Vector3) -> Self;
    fn from_axis_angle(axis: Vector3, angle: f32) -> Self;
    fn to_vector(self) -> Vector3;
    fn mul(self, rhs: Self) -> Self;
    fn conjugate(self) -> Self;
    fn dot(self, vector: Vector3) -> f32;
    fn cross(self, vector: Vector3) -> Vector3;
    fn rotate(self, vector: Vector3) -> Vector3;
}

impl QuaternionExt for Quaternion {
    fn from_vector(vector: Vector3) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: 0.0,
        }
    }

    fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let half = (angle % TAU) / 2.0;
        let s = half.sin();
        let axis = axis.normalize();
        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: half.cos(),
        }
    }

    fn to_vector(self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }

    fn conjugate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    fn dot(self, vector: Vector3) -> f32 {
        vector.x * self.x + vector.y * self.y + vector.z * self.z
    }

    fn cross(self, vector: Vector3) -> Vector3 {
        Vector3 {
            x: vector.y * self.z + vector.z * self.y,
            y: vector.z * self.x + vector.x * self.z,
            z: vector.x * self.y + vector.y * self.x,
        }
    }

    fn rotate(self, vector: Vector3) -> Vector3 {
        let qv = Self::from_vector(vector);
        self.mul(qv).mul(self.conjugate()).to_vector()
    }
}
