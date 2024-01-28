use std::fmt::Display;

use crate::raudio::Raudio;
use raylib_ffi::*;

pub trait WaveExt: Sized {
    fn load(filename: impl Display) -> Result<Self, String>;
    fn load_from_memory(tpe: WaveType, data: Vec<u8>) -> Result<Self, String>;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn export(self, filename: impl Display) -> bool;
    fn export_as_code(self, filename: impl Display) -> bool;
    fn copy(self) -> Self;
    fn crop(&mut self, init_sample: i32, final_sample: i32);
    fn format(&mut self, sample_rate: i32, sample_size: i32, channels: i32);
    fn load_samples(self) -> Result<Vec<f32>, String>;
}

impl WaveExt for Wave {
    fn load(filename: impl Display) -> Result<Self, String> {
        Raudio::__load_wave(filename)
    }

    fn load_from_memory(tpe: WaveType, data: Vec<u8>) -> Result<Self, String> {
        Raudio::__load_wave_from_memory(tpe, data)
    }

    fn is_ready(self) -> bool {
        Raudio::__is_wave_ready(self)
    }

    fn unload(self) {
        Raudio::__unload_wave(self)
    }

    fn export(self, filename: impl Display) -> bool {
        Raudio::__export_wave(self, filename)
    }

    fn export_as_code(self, filename: impl Display) -> bool {
        Raudio::__export_wave_as_code(self, filename)
    }

    fn copy(self) -> Self {
        Raudio::__wave_copy(self)
    }

    fn crop(&mut self, init_sample: i32, final_sample: i32) {
        Raudio::__wave_crop(self, init_sample, final_sample)
    }

    fn format(&mut self, sample_rate: i32, sample_size: i32, channels: i32) {
        Raudio::__wave_format(self, sample_rate, sample_size, channels)
    }

    fn load_samples(self) -> Result<Vec<f32>, String> {
        Raudio::__load_wave_samples(self)
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
