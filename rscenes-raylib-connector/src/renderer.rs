use crate::rcore::Rcore;
use crate::vector::*;
use raylib_ffi::*;

#[derive(Clone, Copy, Debug)]
pub struct Renderer {
    pub rcore: Rcore,
}

impl Renderer {
    pub fn new() -> Self {
        Self { rcore: Rcore }
    }

    #[allow(non_snake_case)]
    pub fn default_camera_2D(&self) -> Camera2D {
        Camera2D {
            offset: Vector2::ZERO,
            target: Vector2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
        }
    }

    #[allow(non_snake_case)]
    pub fn default_camera_3D(&self) -> Camera3D {
        Camera3D {
            position: Vector3::BACK,
            target: Vector3::ZERO,
            up: Vector3::UP,
            fovy: 100.0,
            projection: enums::CameraProjection::Perspective as i32,
        }
    }

    #[must_use]
    #[allow(non_snake_case)]
    pub fn mode_2D<F, R, E>(&self, camera: Camera2D, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.rcore.begin_mode_2D(camera);
        let res = block();
        self.rcore.end_mode_2D();
        res
    }

    #[must_use]
    #[allow(non_snake_case)]
    pub fn mode_3D<F, R, E>(&self, camera: Camera3D, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.rcore.begin_mode_3D(camera);
        let res = block();
        self.rcore.end_mode_3D();
        res
    }

    #[must_use]
    pub fn texture_mode<F, R, E>(&self, target: RenderTexture2D, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.rcore.begin_texture_mode(target);
        let res = block();
        self.rcore.end_texture_mode();
        res
    }

    #[must_use]
    pub fn shader_mode<F, R, E>(&self, shader: Shader, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.rcore.begin_shader_mode(shader);
        let res = block();
        self.rcore.end_shader_mode();
        res
    }

    #[must_use]
    pub fn blend_mode<F, R, E>(&self, mode: i32, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.rcore.begin_blend_mode(mode);
        let res = block();
        self.rcore.end_blend_mode();
        res
    }

    #[must_use]
    pub fn scissor_mode<F, R, E>(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        block: F,
    ) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.rcore.begin_scissor_mode(x, y, width, height);
        let res = block();
        self.rcore.end_scissor_mode();
        res
    }

    #[must_use]
    pub fn vr_stereo_mode<F, R, E>(&self, config: VrStereoConfig, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.rcore.begin_vr_stereo_mode(config);
        let res = block();
        self.rcore.end_vr_stereo_mode();
        res
    }

    // TODO: record automation event
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn should_export_rcore() {
        let renderer = Renderer::new();
        assert_eq!(renderer.rcore.type_id(), TypeId::of::<Rcore>())
    }
}
