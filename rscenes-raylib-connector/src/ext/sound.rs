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
    fn play(self);
    fn stop(self);
    fn pause(self);
    fn resume(self);
    fn is_playing(self) -> bool;
    fn set_volume(self, volume: f32);
    fn set_pitch(self, pitch: f32);
    fn set_pan(self, pan: f32);
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

    fn play(self) {
        Raudio::__play_sound(self)
    }

    fn stop(self) {
        Raudio::__stop_sound(self)
    }

    fn pause(self) {
        Raudio::__pause_sound(self)
    }

    fn resume(self) {
        Raudio::__resume_sound(self)
    }

    fn is_playing(self) -> bool {
        Raudio::__is_sound_playing(self)
    }

    fn set_volume(self, volume: f32) {
        Raudio::__set_sound_volume(self, volume)
    }

    fn set_pitch(self, pitch: f32) {
        Raudio::__set_sound_pitch(self, pitch)
    }

    fn set_pan(self, pan: f32) {
        Raudio::__set_sound_pan(self, pan)
    }
}
