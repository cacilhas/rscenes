use super::vector::Vector3Ext;
use raylib_ffi::{Quaternion, Vector3};

pub trait QuaternionExt: Sized {
    fn to_vector(self) -> Vector3;
    fn mul(self, rhs: Self) -> Self;
    fn conjugate(self) -> Self;
    fn dot(self, vector: Vector3) -> f32;
    fn cross(self, vector: Vector3) -> Vector3;
    fn rotate(self, vector: Vector3) -> Vector3;
}

impl QuaternionExt for Quaternion {
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
        let qv = vector.to_quaternion(0.0);
        self.mul(qv).mul(self.conjugate()).to_vector()
    }
}
