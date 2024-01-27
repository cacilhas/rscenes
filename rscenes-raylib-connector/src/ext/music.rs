use crate::raudio::Raudio;
use raylib_ffi::*;
use std::fmt::Display;

pub trait MusicExt: Sized {
    fn load(filename: impl Display) -> Result<Self, String>;
    fn load_from_memory(tpe: impl Display, data: Vec<u8>) -> Result<Self, String>;

    fn is_ready(self) -> bool;
    fn unload(self);
    fn play(self);
    fn is_playing(self) -> bool;
    fn update(self);
    fn stop(self);
    fn pause(self);
    fn resume(self);
    fn seek(self, position: f32);
    fn set_volume(self, volume: f32);
    fn set_pitch(self, pitch: f32);
    fn set_pan(self, pan: f32);
    fn get_time_length(self) -> f32;
    fn get_time_played(self) -> f32;
}

impl MusicExt for Music {
    fn load(filename: impl Display) -> Result<Self, String> {
        Raudio::__load_music_stream(filename)
    }

    fn load_from_memory(tpe: impl Display, data: Vec<u8>) -> Result<Self, String> {
        Raudio::__load_music_stream_from_memory(tpe, data)
    }

    fn is_ready(self) -> bool {
        Raudio::__is_music_ready(self)
    }

    fn unload(self) {
        Raudio::__unload_music_stream(self)
    }

    fn play(self) {
        Raudio::__play_music_stream(self)
    }

    fn is_playing(self) -> bool {
        Raudio::__is_music_stream_playing(self)
    }

    fn update(self) {
        Raudio::__update_music_stream(self)
    }

    fn stop(self) {
        Raudio::__stop_music_stream(self)
    }

    fn pause(self) {
        Raudio::__pause_music_stream(self)
    }

    fn resume(self) {
        Raudio::__resume_music_stream(self)
    }

    fn seek(self, position: f32) {
        Raudio::__seek_music_stream(self, position)
    }

    fn set_volume(self, volume: f32) {
        Raudio::__set_music_volume(self, volume)
    }

    fn set_pitch(self, pitch: f32) {
        Raudio::__set_music_pitch(self, pitch)
    }

    fn set_pan(self, pan: f32) {
        Raudio::__set_music_pan(self, pan)
    }

    fn get_time_length(self) -> f32 {
        Raudio::__get_music_time_length(self)
    }

    fn get_time_played(self) -> f32 {
        Raudio::__get_music_time_played(self)
    }
}
