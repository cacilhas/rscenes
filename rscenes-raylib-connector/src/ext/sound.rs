use crate::raudio::Raudio;
use raylib_ffi::*;
use std::fmt::Display;

pub trait SoundExt: Sized {
    /// Load sound from file
    fn load(filename: impl Display) -> Result<Self, String>;
    /// Load sound from wave data
    fn load_from_wave(wave: Wave) -> Self;

    /// Create a new sound that shares the same sample data as the source sound, does not own the sound data
    fn alias(self) -> Self;
    /// Check whether a sound is ready
    fn is_ready(self) -> bool;
    /// Unload wave data
    fn unload(self);
    /// Unload a sound alias (does not deallocate sample data)
    fn unload_alias(self);
    /// Play a sound
    fn play(self);
    /// Stop playing a sound
    fn stop(self);
    /// Pause a sound
    fn pause(self);
    /// Resume a paused sound
    fn resume(self);
    /// Check whether a sound is currently playing
    fn is_playing(self) -> bool;
    /// Set volume for a sound (1.0 is max level)
    fn set_volume(self, volume: f32);
    /// Set pitch for a sound (1.0 is base level)
    fn set_pitch(self, pitch: f32);
    /// Set pan for a sound (0.5 is center)
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
