use raylib_ffi::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Raudio;

/// Crate only methods
impl Raudio {
    // Audio device management

    pub(crate) fn __init_audio_device() {
        unsafe { InitAudioDevice() }
    }

    pub(crate) fn __close_audio_device() {
        unsafe { CloseAudioDevice() }
    }

    pub(crate) fn __is_audio_device_ready() -> bool {
        unsafe { IsAudioDeviceReady() }
    }

    pub(crate) fn __set_master_volume(volume: f32) {
        unsafe { SetMasterVolume(volume) }
    }

    pub(crate) fn __get_master_volume() -> f32 {
        unsafe { GetMasterVolume() }
    }

    // Wave/Sound loading/unloading methods
}

/// Exported methods
impl Raudio {
    // Audio device management

    pub fn init_audio_device(&self) {
        Self::__init_audio_device()
    }

    pub fn close_audio_device(&self) {
        Self::__close_audio_device()
    }

    pub fn is_audio_device_ready(&self) -> bool {
        Self::__is_audio_device_ready()
    }

    pub fn set_master_volume(&self, volume: f32) {
        Self::__set_master_volume(volume)
    }

    pub fn get_master_volume(&self) -> f32 {
        Self::__get_master_volume()
    }

    // Wave/Sound loading/unloading methods
}