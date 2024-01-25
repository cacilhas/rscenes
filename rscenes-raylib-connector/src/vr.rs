use raylib_ffi as rl;

#[derive(Debug)]
pub struct VrStereoConfig(pub(crate) rl::VrStereoConfig);

impl VrStereoConfig {
    pub fn load(device: rl::VrDeviceInfo) -> Self {
        unsafe { Self(rl::LoadVrStereoConfig(device)) }
    }
}

impl Drop for VrStereoConfig {
    fn drop(&mut self) {
        unsafe { rl::UnloadVrStereoConfig(self.0) }
    }
}
