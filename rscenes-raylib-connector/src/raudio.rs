use raylib_ffi::*;
use std::fmt::Display;

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

    pub(crate) fn __load_wave(filename: impl Display) -> Result<Wave, String> {
        unsafe {
            let wave = LoadWave(rl_str!(filename));
            if wave.data.is_null() {
                Err(format!("error loading wave from {}", filename))
            } else {
                Ok(wave)
            }
        }
    }

    pub(crate) fn __load_wave_from_memory(
        tpe: impl Display,
        data: Vec<u8>,
    ) -> Result<Wave, String> {
        unsafe {
            let size = data.len() as i32;
            let wave = LoadWaveFromMemory(rl_str!(tpe), data.as_ptr(), size);
            if wave.data.is_null() {
                Err("error loading wave from memory".to_owned())
            } else {
                Ok(wave)
            }
        }
    }

    pub(crate) fn __is_wave_ready(wave: Wave) -> bool {
        unsafe { IsWaveReady(wave) }
    }

    pub(crate) fn __load_sound(filename: impl Display) -> Result<Sound, String> {
        unsafe {
            let sound = LoadSound(rl_str!(filename));
            if sound.stream.buffer.is_null() {
                Err(format!("error loading sound from {}", filename))
            } else {
                Ok(sound)
            }
        }
    }

    pub(crate) fn __load_sound_from_wave(wave: Wave) -> Sound {
        unsafe { LoadSoundFromWave(wave) }
    }

    pub(crate) fn __load_sound_alias(source: Sound) -> Sound {
        unsafe { LoadSoundAlias(source) }
    }

    pub(crate) fn __is_sound_ready(sound: Sound) -> bool {
        unsafe { IsSoundReady(sound) }
    }

    pub(crate) fn __unload_wave(wave: Wave) {
        unsafe { UnloadWave(wave) }
    }

    pub(crate) fn __unload_sound(sound: Sound) {
        unsafe { UnloadSound(sound) }
    }

    pub(crate) fn __unload_sound_alias(sound: Sound) {
        unsafe { UnloadSoundAlias(sound) }
    }

    pub(crate) fn __export_wave(wave: Wave, filename: impl Display) -> bool {
        unsafe { ExportWave(wave, rl_str!(filename)) }
    }

    pub(crate) fn __export_wave_as_code(wave: Wave, filename: impl Display) -> bool {
        unsafe { ExportWaveAsCode(wave, rl_str!(filename)) }
    }

    // Wave/Sound management methods
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

    pub fn load_wave(&self, filename: impl Display) -> Result<Wave, String> {
        Self::__load_wave(filename)
    }

    pub fn load_wave_from_memory(&self, tpe: impl Display, data: Vec<u8>) -> Result<Wave, String> {
        Self::__load_wave_from_memory(tpe, data)
    }

    pub fn is_wave_ready(&self, wave: Wave) -> bool {
        Self::__is_wave_ready(wave)
    }

    pub fn load_sound(&self, filename: impl Display) -> Result<Sound, String> {
        Self::__load_sound(filename)
    }

    pub fn load_sound_from_wave(&self, wave: Wave) -> Sound {
        Self::__load_sound_from_wave(wave)
    }

    pub fn load_sound_alias(&self, source: Sound) -> Sound {
        Self::__load_sound_alias(source)
    }

    pub fn is_sound_ready(&self, sound: Sound) -> bool {
        Self::__is_sound_ready(sound)
    }

    pub fn unload_wave(&self, wave: Wave) {
        Self::__unload_wave(wave)
    }

    pub fn unload_sound(&self, sound: Sound) {
        Self::__unload_sound(sound)
    }

    pub fn unload_sound_alias(&self, sound: Sound) {
        Self::__unload_sound_alias(sound)
    }

    pub fn export_wave(&self, wave: Wave, filename: impl Display) -> bool {
        Self::__export_wave(wave, filename)
    }

    pub fn export_wave_as_code(&self, wave: Wave, filename: impl Display) -> bool {
        Self::__export_wave_as_code(wave, filename)
    }

    // Wave/Sound management methods
}
