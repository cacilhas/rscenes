use super::vector::Vector3Ext;
use raylib_ffi::*;

pub trait MatrixExt {
    fn from_vectors(front: Vector3, up: Vector3) -> Self;
    fn mul(self, vector: Quaternion) -> Quaternion;
}

impl MatrixExt for Matrix {
    fn from_vectors(front: Vector3, up: Vector3) -> Self {
        let right = front.cross(up).normalize();
        let up = right.cross(front).normalize();
        Self {
            m0: right.x,
            m4: up.x,
            m8: front.x,
            m12: 0.0,
            m1: right.y,
            m5: up.y,
            m9: front.y,
            m13: 0.0,
            m2: right.z,
            m6: up.z,
            m10: front.z,
            m14: 0.0,
            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    fn mul(self, vector: Quaternion) -> Quaternion {
        Quaternion {
            x: self.m0 * vector.x + self.m4 * vector.y + self.m8 * vector.z + self.m12 * vector.w,
            y: self.m1 * vector.x + self.m5 * vector.y + self.m9 * vector.z + self.m13 * vector.w,
            z: self.m2 * vector.x + self.m6 * vector.y + self.m10 * vector.z + self.m14 * vector.w,
            w: self.m3 * vector.x + self.m7 * vector.y + self.m11 * vector.z + self.m15 * vector.w,
        }
    }
}
