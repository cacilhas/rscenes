use crate::raudio::RaudioImpl;
use raylib_ffi::*;
use std::fmt::Display;

pub trait MusicExt: Sized {
    /// Load music stream from file
    fn load(filename: impl Display) -> Result<Self, String>;
    /// Load music stream from data
    fn load_from_memory(tpe: impl Display, data: &[u8]) -> Result<Self, String>;

    /// Check whether a music stream is ready
    fn is_ready(self) -> bool;
    /// Unload music stream
    fn unload(self);
    /// Start music playing
    fn play(self);
    /// Check whether music is playing
    fn is_playing(self) -> bool;
    /// Updates buffers for music streaming
    fn update(self);
    /// Stop music playing
    fn stop(self);
    /// Pause music playing
    fn pause(self);
    /// Resume playing paused music
    fn resume(self);
    /// Seek music to a position (in seconds)
    fn seek(self, position: f32);
    /// Set volume for music (1.0 is max level)
    fn set_volume(self, volume: f32);
    /// Set pitch for a music (1.0 is base level)
    fn set_pitch(self, pitch: f32);
    /// Set pan for a music (0.5 is center)
    fn set_pan(self, pan: f32);
    /// Get music time length (in seconds)
    fn get_time_length(self) -> f32;
    /// Get current music time played (in seconds)
    fn get_time_played(self) -> f32;
}

impl MusicExt for Music {
    fn load(filename: impl Display) -> Result<Self, String> {
        RaudioImpl::__load_music_stream(filename)
    }

    fn load_from_memory(tpe: impl Display, data: &[u8]) -> Result<Self, String> {
        RaudioImpl::__load_music_stream_from_memory(tpe, data)
    }

    fn is_ready(self) -> bool {
        RaudioImpl::__is_music_ready(self)
    }

    fn unload(self) {
        RaudioImpl::__unload_music_stream(self)
    }

    fn play(self) {
        RaudioImpl::__play_music_stream(self)
    }

    fn is_playing(self) -> bool {
        RaudioImpl::__is_music_stream_playing(self)
    }

    fn update(self) {
        RaudioImpl::__update_music_stream(self)
    }

    fn stop(self) {
        RaudioImpl::__stop_music_stream(self)
    }

    fn pause(self) {
        RaudioImpl::__pause_music_stream(self)
    }

    fn resume(self) {
        RaudioImpl::__resume_music_stream(self)
    }

    fn seek(self, position: f32) {
        RaudioImpl::__seek_music_stream(self, position)
    }

    fn set_volume(self, volume: f32) {
        RaudioImpl::__set_music_volume(self, volume)
    }

    fn set_pitch(self, pitch: f32) {
        RaudioImpl::__set_music_pitch(self, pitch)
    }

    fn set_pan(self, pan: f32) {
        RaudioImpl::__set_music_pan(self, pan)
    }

    fn get_time_length(self) -> f32 {
        RaudioImpl::__get_music_time_length(self)
    }

    fn get_time_played(self) -> f32 {
        RaudioImpl::__get_music_time_played(self)
    }
}
