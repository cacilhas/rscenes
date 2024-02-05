use raylib_ffi::{Quaternion, Vector3};

pub trait QuaternionExt: Sized {
    fn conjugate(self) -> Self;
    fn dot(self, vector: Vector3) -> f32;
    fn cross(self, vector: Vector3) -> Vector3;
    fn rotate(self, vector: Vector3) -> Vector3;
}

impl QuaternionExt for Quaternion {
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
        let conj = self.conjugate();
        let dot_prod = self.dot(vector);
        let cross_prod = self.cross(vector);

        Vector3 {
            x: conj.w * vector.x
                + conj.x * dot_prod
                + (conj.y * cross_prod.z - conj.z * cross_prod.y),
            y: conj.w * vector.y
                + conj.y * dot_prod
                + (conj.z * cross_prod.x - conj.x * cross_prod.z),
            z: conj.w * vector.z
                + conj.z * dot_prod
                + (conj.x * cross_prod.y - conj.y * cross_prod.x),
        }
    }
}
