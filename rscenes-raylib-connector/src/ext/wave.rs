/// Set texture wrapping mode
use crate::raudio::RaudioImpl;
use raylib_ffi::*;
use std::fmt::Display;

pub trait WaveExt: Sized {
    /// Load wave data from file
    fn load(filename: impl Display) -> Result<Self, String>;
    /// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
    fn load_from_memory(tpe: WaveType, data: &[u8]) -> Result<Self, String>;

    /// Check whether wave data is ready
    fn is_ready(self) -> bool;
    /// Unload wave data
    fn unload(self);
    /// Export wave data to file, returns true on success
    fn export(self, filename: impl Display) -> bool;
    /// Export wave sample data to code (.h), returns true on success
    fn export_as_code(self, filename: impl Display) -> bool;
    /// Copy a wave to a new wave
    fn copy(self) -> Self;
    // Crop a wave to defined samples range
    fn crop(&mut self, init_sample: i32, final_sample: i32);
    /// Convert wave data to desired format
    fn format(&mut self, sample_rate: i32, sample_size: i32, channels: i32);
    /// Load samples data from wave as a 32bit float data array
    fn load_samples(self) -> Result<Vec<f32>, String>;
}

impl WaveExt for Wave {
    fn load(filename: impl Display) -> Result<Self, String> {
        RaudioImpl::__load_wave(filename)
    }

    fn load_from_memory(tpe: WaveType, data: &[u8]) -> Result<Self, String> {
        RaudioImpl::__load_wave_from_memory(tpe, data)
    }

    fn is_ready(self) -> bool {
        RaudioImpl::__is_wave_ready(self)
    }

    fn unload(self) {
        RaudioImpl::__unload_wave(self)
    }

    fn export(self, filename: impl Display) -> bool {
        RaudioImpl::__export_wave(self, filename)
    }

    fn export_as_code(self, filename: impl Display) -> bool {
        RaudioImpl::__export_wave_as_code(self, filename)
    }

    fn copy(self) -> Self {
        RaudioImpl::__wave_copy(self)
    }

    fn crop(&mut self, init_sample: i32, final_sample: i32) {
        RaudioImpl::__wave_crop(self, init_sample, final_sample)
    }

    fn format(&mut self, sample_rate: i32, sample_size: i32, channels: i32) {
        RaudioImpl::__wave_format(self, sample_rate, sample_size, channels)
    }

    fn load_samples(self) -> Result<Vec<f32>, String> {
        RaudioImpl::__load_wave_samples(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WaveType {
    Aac,
    Ape,
    Au,
    Flat,
    Mp3,
    M4a,
    Ogg,
    Opus,
    Qoa,
    Wav,
}

impl Display for WaveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaveType::Aac => f.write_str(".aac"),
            WaveType::Ape => f.write_str(".ape"),
            WaveType::Au => f.write_str(".au"),
            WaveType::Flat => f.write_str(".flat"),
            WaveType::Mp3 => f.write_str(".mp3"),
            WaveType::M4a => f.write_str(".m4a"),
            WaveType::Ogg => f.write_str(".ogg"),
            WaveType::Opus => f.write_str(".opus"),
            WaveType::Qoa => f.write_str(".qoa"),
            WaveType::Wav => f.write_str(".wav"),
        }
    }
}
