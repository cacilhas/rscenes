use crate::raudio::Raudio;
use raylib_ffi::*;

pub trait AudioStreamExt: Sized {
    /// Load audio stream (to stream raw audio pcm data)
    fn load(sample_rate: u32, sample_size: u32, channels: u32) -> Result<Self, String>;
    /// Default size for new audio streams
    fn set_default_buffer_size(size: i32);

    /// Check whether an audio stream is ready
    fn is_ready(self) -> bool;
    /// Unload audio stream and free memory
    fn unload(self);
    /// Update audio stream buffers with data
    fn update(self, data: &[u8]);
    /// Check whether any audio stream buffers requires refill
    fn is_processed(self) -> bool;
    /// Play audio stream
    fn play(self);
    /// Pause audio stream
    fn pause(self);
    /// Resume audio stream
    fn resume(self);
    /// Check whether audio stream is playing
    fn is_playing(self) -> bool;
    /// Stop audio stream
    fn stop(self);
    /// Set volume for audio stream (1.0 is max level)
    fn set_volume(self, volume: f32);
    /// Set pitch for audio stream (1.0 is base level)
    fn set_pitch(self, pitch: f32);
    /// Set pan for audio stream (0.5 is centered)
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

    fn update(self, data: &[u8]) {
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
