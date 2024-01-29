use crate::{ext::wave::WaveType, utils::array_from_c};
use raylib_ffi::*;
use std::{
    ffi::c_void,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RaudioImpl;

/// Crate only methods
impl RaudioImpl {
    // Audio device management

    pub fn __init_audio_device() {
        unsafe { InitAudioDevice() }
    }

    pub fn __close_audio_device() {
        unsafe { CloseAudioDevice() }
    }

    pub fn __is_audio_device_ready() -> bool {
        unsafe { IsAudioDeviceReady() }
    }

    pub fn __set_master_volume(volume: f32) {
        unsafe { SetMasterVolume(volume) }
    }

    pub fn __get_master_volume() -> f32 {
        unsafe { GetMasterVolume() }
    }

    // Wave/Sound loading/unloading methods

    pub fn __load_wave(filename: impl Display) -> Result<Wave, String> {
        unsafe {
            let wave = LoadWave(rl_str!(filename));
            if wave.data.is_null() {
                Err(format!("error loading wave from {}", filename))
            } else {
                Ok(wave)
            }
        }
    }

    pub fn __load_wave_from_memory(tpe: impl Display, data: &[u8]) -> Result<Wave, String> {
        unsafe {
            let size = data.len() as i32;
            let wave = LoadWaveFromMemory(rl_str!(tpe), data.to_owned().as_ptr(), size);
            if wave.data.is_null() {
                Err("error loading wave from memory".to_owned())
            } else {
                Ok(wave)
            }
        }
    }

    pub fn __is_wave_ready(wave: Wave) -> bool {
        unsafe { IsWaveReady(wave) }
    }

    pub fn __load_sound(filename: impl Display) -> Result<Sound, String> {
        unsafe {
            let sound = LoadSound(rl_str!(filename));
            if sound.stream.buffer.is_null() {
                Err(format!("error loading sound from {}", filename))
            } else {
                Ok(sound)
            }
        }
    }

    pub fn __load_sound_from_wave(wave: Wave) -> Sound {
        unsafe { LoadSoundFromWave(wave) }
    }

    pub fn __load_sound_alias(source: Sound) -> Sound {
        unsafe { LoadSoundAlias(source) }
    }

    pub fn __is_sound_ready(sound: Sound) -> bool {
        unsafe { IsSoundReady(sound) }
    }

    pub fn __unload_wave(wave: Wave) {
        unsafe { UnloadWave(wave) }
    }

    pub fn __unload_sound(sound: Sound) {
        unsafe { UnloadSound(sound) }
    }

    pub fn __unload_sound_alias(sound: Sound) {
        unsafe { UnloadSoundAlias(sound) }
    }

    pub fn __export_wave(wave: Wave, filename: impl Display) -> bool {
        unsafe { ExportWave(wave, rl_str!(filename)) }
    }

    pub fn __export_wave_as_code(wave: Wave, filename: impl Display) -> bool {
        unsafe { ExportWaveAsCode(wave, rl_str!(filename)) }
    }

    // Wave/Sound management methods

    pub fn __play_sound(sound: Sound) {
        unsafe { PlaySound(sound) }
    }

    pub fn __stop_sound(sound: Sound) {
        unsafe { StopSound(sound) }
    }

    pub fn __pause_sound(sound: Sound) {
        unsafe { PauseSound(sound) }
    }

    pub fn __resume_sound(sound: Sound) {
        unsafe { ResumeSound(sound) }
    }

    pub fn __is_sound_playing(sound: Sound) -> bool {
        unsafe { IsSoundPlaying(sound) }
    }

    pub fn __set_sound_volume(sound: Sound, volume: f32) {
        unsafe { SetSoundVolume(sound, volume) }
    }

    pub fn __set_sound_pitch(sound: Sound, pitch: f32) {
        unsafe { SetSoundPitch(sound, pitch) }
    }

    pub fn __set_sound_pan(sound: Sound, pan: f32) {
        unsafe { SetSoundPan(sound, pan) }
    }

    pub fn __wave_copy(wave: Wave) -> Wave {
        unsafe { WaveCopy(wave) }
    }

    pub fn __wave_crop(wave: &mut Wave, init_sample: i32, final_sample: i32) {
        unsafe { WaveCrop(wave, init_sample, final_sample) }
    }

    pub fn __wave_format(wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
        unsafe { WaveFormat(wave, sample_rate, sample_size, channels) }
    }

    pub fn __load_wave_samples(wave: Wave) -> Result<Vec<f32>, String> {
        unsafe {
            let raw = LoadWaveSamples(wave);
            // TODO: review this calculation
            let count = wave.frameCount * wave.sampleSize / 32;
            let res = array_from_c(raw, count as usize, || {
                "couldn't load samples from wave".to_owned()
            })?
            .iter()
            .map(|e| *e)
            .collect::<Vec<_>>();
            UnloadWaveSamples(raw);
            Ok(res)
        }
    }

    // Music management methods

    pub fn __load_music_stream(filename: impl Display) -> Result<Music, String> {
        unsafe {
            let music = LoadMusicStream(rl_str!(filename));
            if music.stream.buffer.is_null() {
                Err(format!("failed to load music stream from {}", filename))
            } else {
                Ok(music)
            }
        }
    }

    pub fn __load_music_stream_from_memory(
        tpe: impl Display,
        data: &[u8],
    ) -> Result<Music, String> {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_ptr();
            let music = LoadMusicStreamFromMemory(rl_str!(tpe), data, size);
            if music.stream.buffer.is_null() {
                Err("failed to load music stream from memory".to_owned())
            } else {
                Ok(music)
            }
        }
    }

    pub fn __is_music_ready(music: Music) -> bool {
        unsafe { IsMusicReady(music) }
    }

    pub fn __unload_music_stream(music: Music) {
        unsafe { UnloadMusicStream(music) }
    }

    pub fn __play_music_stream(music: Music) {
        unsafe { PlayMusicStream(music) }
    }

    pub fn __is_music_stream_playing(music: Music) -> bool {
        unsafe { IsMusicStreamPlaying(music) }
    }

    pub fn __update_music_stream(music: Music) {
        unsafe { UpdateMusicStream(music) }
    }

    pub fn __stop_music_stream(music: Music) {
        unsafe { StopMusicStream(music) }
    }

    pub fn __pause_music_stream(music: Music) {
        unsafe { PauseMusicStream(music) }
    }

    pub fn __resume_music_stream(music: Music) {
        unsafe { ResumeMusicStream(music) }
    }

    pub fn __seek_music_stream(music: Music, position: f32) {
        unsafe { SeekMusicStream(music, position) }
    }

    pub fn __set_music_volume(music: Music, volume: f32) {
        unsafe { SetMusicVolume(music, volume) }
    }

    pub fn __set_music_pitch(music: Music, pitch: f32) {
        unsafe { SetMusicPitch(music, pitch) }
    }

    pub fn __set_music_pan(music: Music, pan: f32) {
        unsafe { SetMusicPan(music, pan) }
    }

    pub fn __get_music_time_length(music: Music) -> f32 {
        unsafe { GetMusicTimeLength(music) }
    }

    pub fn __get_music_time_played(music: Music) -> f32 {
        unsafe { GetMusicTimePlayed(music) }
    }

    // AudioStream management methods

    pub fn __load_audio_stream(
        sample_rate: u32,
        sample_size: u32,
        channels: u32,
    ) -> Result<AudioStream, String> {
        unsafe {
            // TODO: test whether audio stream was properly loaded
            Ok(LoadAudioStream(sample_rate, sample_size, channels))
        }
    }

    pub fn __is_audio_stream_ready(stream: AudioStream) -> bool {
        unsafe { IsAudioStreamReady(stream) }
    }

    pub fn __unload_audio_stream(stream: AudioStream) {
        unsafe { UnloadAudioStream(stream) }
    }

    pub fn __update_audio_stream(stream: AudioStream, data: &[u8]) {
        unsafe {
            let count = data.len() as i32;
            let data = data.as_ptr() as *const c_void;
            UpdateAudioStream(stream, data, count)
        }
    }

    pub fn __is_audio_stream_processed(stream: AudioStream) -> bool {
        unsafe { IsAudioStreamProcessed(stream) }
    }

    pub fn __play_audio_stream(stream: AudioStream) {
        unsafe { PlayAudioStream(stream) }
    }

    pub fn __pause_audio_stream(stream: AudioStream) {
        unsafe { PauseAudioStream(stream) }
    }

    pub fn __resume_audio_stream(stream: AudioStream) {
        unsafe { ResumeAudioStream(stream) }
    }

    pub fn __is_audio_stream_playing(stream: AudioStream) -> bool {
        unsafe { IsAudioStreamPlaying(stream) }
    }

    pub fn __stop_audio_stream(stream: AudioStream) {
        unsafe { StopAudioStream(stream) }
    }

    pub fn __set_audio_stream_volume(stream: AudioStream, volume: f32) {
        unsafe { SetAudioStreamVolume(stream, volume) }
    }

    pub fn __set_audio_stream_pitch(stream: AudioStream, pitch: f32) {
        unsafe { SetAudioStreamPitch(stream, pitch) }
    }

    pub fn __set_audio_stream_pan(stream: AudioStream, pan: f32) {
        unsafe { SetAudioStreamPan(stream, pan) }
    }

    pub fn __set_audio_stream_buffer_size_default(size: i32) {
        unsafe { SetAudioStreamBufferSizeDefault(size) }
    }

    // TODO: SetAudioStreamCallback
    // TODO: AttachAudioStreamProcessor
    // TODO: DetachAudioStreamProcessor
    // TODO: AttachAudioMixedProcessor
    // TODO: DetachAudioMixedProcessor
}

/// Exported methods
pub trait Raudio: Debug {
    // Audio device management

    /// Initialize audio device and context
    fn init_audio_device(&self) {
        RaudioImpl::__init_audio_device()
    }

    /// Close the audio device and context
    fn close_audio_device(&self) {
        RaudioImpl::__close_audio_device()
    }

    /// Check whether audio device has been initialized successfully
    fn is_audio_device_ready(&self) -> bool {
        RaudioImpl::__is_audio_device_ready()
    }

    /// Set master volume (listener)
    fn set_master_volume(&self, volume: f32) {
        RaudioImpl::__set_master_volume(volume)
    }

    /// Get master volume (listener)
    fn get_master_volume(&self) -> f32 {
        RaudioImpl::__get_master_volume()
    }

    // Wave/Sound loading/unloading methods

    /// Load wave data from file
    fn load_wave(&self, filename: impl Display) -> Result<Wave, String> {
        RaudioImpl::__load_wave(filename)
    }

    /// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
    fn load_wave_from_memory(&self, tpe: WaveType, data: &[u8]) -> Result<Wave, String> {
        RaudioImpl::__load_wave_from_memory(tpe, data)
    }

    /// Check whether wave data is ready
    fn is_wave_ready(&self, wave: Wave) -> bool {
        RaudioImpl::__is_wave_ready(wave)
    }

    /// Load sound from file
    fn load_sound(&self, filename: impl Display) -> Result<Sound, String> {
        RaudioImpl::__load_sound(filename)
    }

    /// Load sound from wave data
    fn load_sound_from_wave(&self, wave: Wave) -> Sound {
        RaudioImpl::__load_sound_from_wave(wave)
    }

    /// Create a new sound that shares the same sample data as the source sound, does not own the sound data
    fn load_sound_alias(&self, source: Sound) -> Sound {
        RaudioImpl::__load_sound_alias(source)
    }

    /// Check whether a sound is ready
    fn is_sound_ready(&self, sound: Sound) -> bool {
        RaudioImpl::__is_sound_ready(sound)
    }

    /// Unload wave data
    fn unload_wave(&self, wave: Wave) {
        RaudioImpl::__unload_wave(wave)
    }

    /// Unload sound
    fn unload_sound(&self, sound: Sound) {
        RaudioImpl::__unload_sound(sound)
    }

    /// Unload a sound alias (does not deallocate sample data)
    fn unload_sound_alias(&self, sound: Sound) {
        RaudioImpl::__unload_sound_alias(sound)
    }

    /// Export wave data to file, returns true on success
    fn export_wave(&self, wave: Wave, filename: impl Display) -> bool {
        RaudioImpl::__export_wave(wave, filename)
    }

    /// Export wave sample data to code (.h), returns true on success
    fn export_wave_as_code(&self, wave: Wave, filename: impl Display) -> bool {
        RaudioImpl::__export_wave_as_code(wave, filename)
    }

    // Wave/Sound management methods

    /// Play a sound
    fn play_sound(&self, sound: Sound) {
        RaudioImpl::__play_sound(sound)
    }

    /// Stop playing a sound
    fn stop_sound(&self, sound: Sound) {
        RaudioImpl::__stop_sound(sound)
    }

    /// Pause a sound
    fn pause_sound(&self, sound: Sound) {
        RaudioImpl::__pause_sound(sound)
    }

    /// Resume a paused sound
    fn resume_sound(&self, sound: Sound) {
        RaudioImpl::__resume_sound(sound)
    }

    /// Check whether a sound is currently playing
    fn is_sound_playing(&self, sound: Sound) -> bool {
        RaudioImpl::__is_sound_playing(sound)
    }

    /// Set volume for a sound (1.0 is max level)
    fn set_sound_volume(&self, sound: Sound, volume: f32) {
        RaudioImpl::__set_sound_volume(sound, volume)
    }

    /// Set pitch for a sound (1.0 is base level)
    fn set_sound_pitch(&self, sound: Sound, pitch: f32) {
        RaudioImpl::__set_sound_pitch(sound, pitch)
    }

    /// Set pan for a sound (0.5 is center)
    fn set_sound_pan(&self, sound: Sound, pan: f32) {
        RaudioImpl::__set_sound_pan(sound, pan)
    }

    /// Copy a wave to a new wave
    fn wave_copy(&self, wave: Wave) -> Wave {
        RaudioImpl::__wave_copy(wave)
    }

    // Crop a wave to defined samples range
    fn wave_crop(&self, wave: &mut Wave, init_sample: i32, final_sample: i32) {
        RaudioImpl::__wave_crop(wave, init_sample, final_sample)
    }

    /// Convert wave data to desired format
    fn wave_format(&self, wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
        RaudioImpl::__wave_format(wave, sample_rate, sample_size, channels)
    }

    /// Load samples data from wave as a 32bit float data array
    fn load_wave_samples(&self, wave: Wave) -> Result<Vec<f32>, String> {
        RaudioImpl::__load_wave_samples(wave)
    }

    // Music management methods

    /// Load music stream from file
    fn load_music_stream(&self, filename: impl Display) -> Result<Music, String> {
        RaudioImpl::__load_music_stream(filename)
    }

    /// Load music stream from data
    fn load_music_stream_from_memory(
        &self,
        tpe: impl Display,
        data: &[u8],
    ) -> Result<Music, String> {
        RaudioImpl::__load_music_stream_from_memory(tpe, data)
    }

    /// Check whether a music stream is ready
    fn is_music_ready(&self, music: Music) -> bool {
        RaudioImpl::__is_music_ready(music)
    }

    /// Unload music stream
    fn unload_music_stream(&self, music: Music) {
        RaudioImpl::__unload_music_stream(music)
    }

    /// Start music playing
    fn play_music_stream(&self, music: Music) {
        RaudioImpl::__play_music_stream(music)
    }

    /// Check whether music is playing
    fn is_music_stream_playing(&self, music: Music) -> bool {
        RaudioImpl::__is_music_stream_playing(music)
    }

    /// Updates buffers for music streaming
    fn update_music_stream(&self, music: Music) {
        RaudioImpl::__update_music_stream(music)
    }

    /// Stop music playing
    fn stop_music_stream(&self, music: Music) {
        RaudioImpl::__stop_music_stream(music)
    }

    /// Pause music playing
    fn pause_music_stream(&self, music: Music) {
        RaudioImpl::__pause_music_stream(music)
    }

    /// Resume playing paused music
    fn resume_music_stream(&self, music: Music) {
        RaudioImpl::__resume_music_stream(music)
    }

    /// Seek music to a position (in seconds)
    fn seek_music_stream(&self, music: Music, position: f32) {
        RaudioImpl::__seek_music_stream(music, position)
    }

    /// Set volume for music (1.0 is max level)
    fn set_music_volume(&self, music: Music, volume: f32) {
        RaudioImpl::__set_music_volume(music, volume)
    }

    /// Set pitch for a music (1.0 is base level)
    fn set_music_pitch(&self, music: Music, pitch: f32) {
        RaudioImpl::__set_music_pitch(music, pitch)
    }

    /// Set pan for a music (0.5 is center)
    fn set_music_pan(&self, music: Music, pan: f32) {
        RaudioImpl::__set_music_pan(music, pan)
    }

    /// Get music time length (in seconds)
    fn get_music_time_length(&self, music: Music) -> f32 {
        RaudioImpl::__get_music_time_length(music)
    }

    /// Get current music time played (in seconds)
    fn get_music_time_played(&self, music: Music) -> f32 {
        RaudioImpl::__get_music_time_played(music)
    }

    // AudioStream management methods

    /// Load audio stream (to stream raw audio pcm data)
    fn load_audio_stream(
        &self,
        sample_rate: u32,
        sample_size: u32,
        channels: u32,
    ) -> Result<AudioStream, String> {
        RaudioImpl::__load_audio_stream(sample_rate, sample_size, channels)
    }

    /// Check whether an audio stream is ready
    fn is_audio_stream_ready(&self, stream: AudioStream) -> bool {
        RaudioImpl::__is_audio_stream_ready(stream)
    }

    /// Unload audio stream and free memory
    fn unload_audio_stream(&self, stream: AudioStream) {
        RaudioImpl::__unload_audio_stream(stream)
    }

    /// Update audio stream buffers with data
    fn update_audio_stream(&self, stream: AudioStream, data: &[u8]) {
        RaudioImpl::__update_audio_stream(stream, data)
    }

    /// Check whether any audio stream buffers requires refill
    fn is_audio_stream_processed(&self, stream: AudioStream) -> bool {
        RaudioImpl::__is_audio_stream_processed(stream)
    }

    /// Play audio stream
    fn play_audio_stream(&self, stream: AudioStream) {
        RaudioImpl::__play_audio_stream(stream)
    }

    /// Pause audio stream
    fn pause_audio_stream(&self, stream: AudioStream) {
        RaudioImpl::__pause_audio_stream(stream)
    }

    /// Resume audio stream
    fn resume_audio_stream(&self, stream: AudioStream) {
        RaudioImpl::__resume_audio_stream(stream)
    }

    /// Check whether audio stream is playing
    fn is_audio_stream_playing(&self, stream: AudioStream) -> bool {
        RaudioImpl::__is_audio_stream_playing(stream)
    }

    /// Stop audio stream
    fn stop_audio_stream(&self, stream: AudioStream) {
        RaudioImpl::__stop_audio_stream(stream)
    }

    /// Set volume for audio stream (1.0 is max level)
    fn set_audio_stream_volume(&self, stream: AudioStream, volume: f32) {
        RaudioImpl::__set_audio_stream_volume(stream, volume)
    }

    /// Set pitch for audio stream (1.0 is base level)
    fn set_audio_stream_pitch(&self, stream: AudioStream, pitch: f32) {
        RaudioImpl::__set_audio_stream_pitch(stream, pitch)
    }

    /// Set pan for audio stream (0.5 is centered)
    fn set_audio_stream_pan(&self, stream: AudioStream, pan: f32) {
        RaudioImpl::__set_audio_stream_pan(stream, pan)
    }

    /// Default size for new audio streams
    fn set_audio_stream_buffer_size_default(&self, size: i32) {
        RaudioImpl::__set_audio_stream_buffer_size_default(size)
    }
}
