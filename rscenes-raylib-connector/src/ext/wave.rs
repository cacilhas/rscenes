use std::fmt::Display;

use crate::raudio::Raudio;
use raylib_ffi::*;

pub trait WaveExt: Sized {
    fn load(filename: impl Display) -> Result<Self, String>;
    fn load_from_memory(tpe: impl Display, data: Vec<u8>) -> Result<Self, String>;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn export(self, filename: impl Display) -> bool;
    fn export_as_code(self, filename: impl Display) -> bool;
}

impl WaveExt for Wave {
    fn load(filename: impl Display) -> Result<Self, String> {
        Raudio::__load_wave(filename)
    }

    fn load_from_memory(tpe: impl Display, data: Vec<u8>) -> Result<Self, String> {
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
}
