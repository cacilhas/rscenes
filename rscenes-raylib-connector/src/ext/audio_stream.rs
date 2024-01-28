use crate::raudio::Raudio;
use raylib_ffi::*;

pub trait AudioStreamExt: Sized {
    fn load(sample_rate: u32, sample_size: u32, channels: u32) -> Result<Self, String>;
    fn set_default_buffer_size(size: i32);

    fn is_ready(self) -> bool;
    fn unload(self);
    fn update(self, data: Vec<u8>);
    fn is_processed(self) -> bool;
    fn play(self);
    fn pause(self);
    fn resume(self);
    fn is_playing(self) -> bool;
    fn stop(self);
    fn set_volume(self, volume: f32);
    fn set_pitch(self, pitch: f32);
    fn set_pan(self, pan: f32);
}

impl AudioStreamExt for AudioStream {
    fn load(sample_rate: u32, sample_size: u32, channels: u32) -> Result<Self, String> {
        Raudio::__load_audio_stream(sample_rate, sample_size, channels)
    }

    fn set_default_buffer_size(size: i32) {
        Raudio::__set_audio_stream_buffer_size_default(size)
    }

    fn is_ready(self) -> bool {
        Raudio::__is_audio_stream_ready(self)
    }

    fn unload(self) {
        Raudio::__unload_audio_stream(self)
    }

    fn update(self, data: Vec<u8>) {
        Raudio::__update_audio_stream(self, data)
    }

    fn is_processed(self) -> bool {
        Raudio::__is_audio_stream_processed(self)
    }

    fn play(self) {
        Raudio::__play_audio_stream(self)
    }

    fn pause(self) {
        Raudio::__pause_audio_stream(self)
    }

    fn resume(self) {
        Raudio::__resume_audio_stream(self)
    }

    fn is_playing(self) -> bool {
        Raudio::__is_audio_stream_playing(self)
    }

    fn stop(self) {
        Raudio::__stop_audio_stream(self)
    }

    fn set_volume(self, volume: f32) {
        Raudio::__set_audio_stream_volume(self, volume)
    }

    fn set_pitch(self, pitch: f32) {
        Raudio::__set_audio_stream_pitch(self, pitch)
    }

    fn set_pan(self, pan: f32) {
        Raudio::__set_audio_stream_pan(self, pan)
    }
}
