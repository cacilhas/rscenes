use crate::raudio::Raudio;
use raylib_ffi::*;
use std::fmt::Display;

pub trait SoundExt: Sized {
    fn load(filename: impl Display) -> Result<Self, String>;
    fn load_from_wave(wave: Wave) -> Self;

    fn alias(self) -> Self;
    fn is_ready(self) -> bool;
    fn unload(self);
    fn unload_alias(self);
}

impl SoundExt for Sound {
    fn load(filename: impl Display) -> Result<Self, String> {
        Raudio::__load_sound(filename)
    }

    fn load_from_wave(wave: Wave) -> Self {
        Raudio::__load_sound_from_wave(wave)
    }

    fn alias(self) -> Self {
        Raudio::__load_sound_alias(self)
    }

    fn is_ready(self) -> bool {
        Raudio::__is_sound_ready(self)
    }

    fn unload(self) {
        Raudio::__unload_sound(self)
    }

    fn unload_alias(self) {
        Raudio::__unload_sound_alias(self)
    }
}
