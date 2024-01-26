pub use crate::raudio::Raudio;
pub use crate::rcamera::Rcamera;
pub use crate::rcore::Rcore;
pub use crate::rgestures::Rgestures;
pub use crate::rmodels::Rmodels;
pub use crate::rshapes::Rshapes;
pub use crate::rtext::Rtext;
pub use crate::rtextures::Rtextures;
use raylib_ffi::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct RaylibConnector {
    pub rcore: Rcore,
    pub rgestures: Rgestures,
    pub rcamera: Rcamera,
    pub rshapes: Rshapes,
    pub rtextures: Rtextures,
    pub rtext: Rtext,
    pub rmodels: Rmodels,
    pub raudio: Raudio,
}

/// TODO: the methods implemented bellow should be in the rscenes crate
impl RaylibConnector {
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
    use super::*;
    use std::any::{Any, TypeId};

    #[test]
    fn should_export_rcore() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rcore.type_id(), TypeId::of::<Rcore>())
    }

    #[test]
    fn should_export_rgestures() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rgestures.type_id(), TypeId::of::<Rgestures>())
    }

    #[test]
    fn should_export_rcamera() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rcamera.type_id(), TypeId::of::<Rcamera>())
    }

    #[test]
    fn should_export_rshapes() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rshapes.type_id(), TypeId::of::<Rshapes>())
    }

    #[test]
    fn should_export_rtextures() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rtextures.type_id(), TypeId::of::<Rtextures>())
    }

    #[test]
    fn should_export_rtext() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rtext.type_id(), TypeId::of::<Rtext>())
    }

    #[test]
    fn should_export_rmodels() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rmodels.type_id(), TypeId::of::<Rmodels>())
    }

    #[test]
    fn should_export_raudio() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.raudio.type_id(), TypeId::of::<Raudio>())
    }
}
