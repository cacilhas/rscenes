use super::quaternion::QuaternionExt;
use raylib_ffi::{Quaternion, Vector2, Vector3};
use std::f32::consts::TAU;

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
        let angle = angle % TAU;
        if angle * axis.sqr_length() * self.sqr_length() == 0.0 {
            // If any of self, angle, or axis is zero, there's nothing to do
            return self;
        }
        Quaternion::from_axis_angle(axis, angle).rotate(self)
    }

    fn sqr_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(self) -> f32 {
        self.sqr_length().sqrt()
    }
}
