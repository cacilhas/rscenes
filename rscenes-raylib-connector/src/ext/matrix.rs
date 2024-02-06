use raylib_ffi::*;

pub trait MatrixExt {
    fn mul(self, vector: Quaternion) -> Quaternion;
}

impl MatrixExt for Matrix {
    fn mul(self, vector: Quaternion) -> Quaternion {
        Quaternion {
            x: self.m0 * vector.x + self.m4 * vector.y + self.m8 * vector.z + self.m12 * vector.w,
            y: self.m1 * vector.x + self.m5 * vector.y + self.m9 * vector.z + self.m13 * vector.w,
            z: self.m2 * vector.x + self.m6 * vector.y + self.m10 * vector.z + self.m14 * vector.w,
            w: self.m3 * vector.x + self.m7 * vector.y + self.m11 * vector.z + self.m15 * vector.w,
        }
    }
}
