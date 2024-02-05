use std::f32::consts::TAU;

pub use raylib_ffi::{Vector2, Vector3};

pub trait Vector2Ext: Sized {
    const ZERO: Self;
    const ONE: Self;
    const UP: Self;
    const DOWN: Self;
    const LEFT: Self;
    const RIGHT: Self;
    const N: Self;
    const S: Self;
    const W: Self;
    const E: Self;

    fn mul(self, value: f32) -> Self;
    fn add(self, rhs: Self) -> Self;
    fn eq(self, rhs: Self) -> bool;
    fn normalize(self) -> Self;
    fn rotate(self, angle: f32) -> Self;
    fn sqr_length(self) -> f32;
    fn length(self) -> f32;
}

impl Vector2Ext for Vector2 {
    const ZERO: Self = Self { x: 0.0, y: 0.0 };
    const ONE: Self = Self { x: 1.0, y: 1.0 };
    const UP: Self = Self { x: 0.0, y: -1.0 };
    const DOWN: Self = Self { x: 0.0, y: 1.0 };
    const LEFT: Self = Self { x: -1.0, y: 0.0 };
    const RIGHT: Self = Self { x: 1.0, y: 0.0 };
    const N: Self = Self::UP;
    const S: Self = Self::DOWN;
    const W: Self = Self::LEFT;
    const E: Self = Self::RIGHT;

    fn mul(self, value: f32) -> Self {
        Self {
            x: self.x * value,
            y: self.y * value,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    fn eq(self, rhs: Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }

    fn normalize(self) -> Self {
        let angle = self.y.atan2(self.x);
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    fn rotate(self, angle: f32) -> Self {
        let current_angle = self.y.atan2(self.x);
        let length = self.length();
        let desired_angle = (current_angle + angle) % TAU;
        Self {
            x: desired_angle.cos() * length,
            y: desired_angle.sin() * length,
        }
    }

    fn sqr_length(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn length(self) -> f32 {
        self.sqr_length().sqrt()
    }
}

pub trait Vector3Ext {
    const ZERO: Self;
    const ONE: Self;
    const FORTH: Self;
    const BACK: Self;
    const LEFT: Self;
    const RIGHT: Self;
    const UP: Self;
    const DOWN: Self;

    fn mul(self, value: f32) -> Self;
    fn add(self, rhs: Self) -> Self;
    fn eq(self, rhs: Self) -> bool;
    fn normalize(self) -> Self;
    fn rotate(self, angle: f32, axis: Self) -> Self;
    fn sqr_length(self) -> f32;
    fn length(self) -> f32;
}

impl Vector3Ext for Vector3 {
    const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    const FORTH: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    const BACK: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    const LEFT: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    const RIGHT: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    const DOWN: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };

    fn mul(self, value: f32) -> Self {
        Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    fn eq(self, rhs: Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }

    fn normalize(self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    fn rotate(self, angle: f32, axis: Self) -> Self {
        if angle * axis.sqr_length() * self.sqr_length() == 0.0 {
            // If any of self, angle, or axis is zero, there's nothing to do
            return self;
        }

        // TODO: it'll be way easier using raymath

        let axis = axis.normalize();

        // Calculate the sine and cosine of half the angle
        let half = angle * 0.5;
        let sin_half = half.sin();
        let cos_half = half.cos();

        // Create the quaternion for the rotation
        let w = cos_half;
        let x = axis.x * sin_half;
        let y = axis.y * sin_half;
        let z = axis.z * sin_half;

        // Conjugate the quaternion
        let conj_w = w;
        let conj_x = -x;
        let conj_y = -y;
        let conj_z = -z;

        // Apply the quaternion rotation to the vector
        let dot_prod = self.x * x + self.y * y + self.z * z;
        let cross_prod_x = self.y * z + self.z * y;
        let cross_prod_y = self.z * x + self.x * z;
        let cross_prod_z = self.x * y + self.y * x;

        Self {
            x: conj_w * self.x
                + conj_x * dot_prod
                + (conj_y * cross_prod_z - conj_z * cross_prod_y),
            y: conj_w * self.y
                + conj_y * dot_prod
                + (conj_z * cross_prod_x - conj_x * cross_prod_z),
            z: conj_w * self.z
                + conj_z * dot_prod
                + (conj_x * cross_prod_y - conj_y * cross_prod_x),
        }
    }

    fn sqr_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(self) -> f32 {
        self.sqr_length().sqrt()
    }
}
