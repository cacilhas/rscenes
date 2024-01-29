use crate::{ext::wave::WaveType, utils::array_from_c};
use raylib_ffi::*;
use std::{ffi::c_void, fmt::Display};

#[derive(Clone, Copy, Debug, Default)]
pub struct Raudio;

/// Crate only methods
impl Raudio {
    // Audio device management

    pub(crate) fn __init_audio_device() {
        unsafe { InitAudioDevice() }
    }

    pub(crate) fn __close_audio_device() {
        unsafe { CloseAudioDevice() }
    }

    pub(crate) fn __is_audio_device_ready() -> bool {
        unsafe { IsAudioDeviceReady() }
    }

    pub(crate) fn __set_master_volume(volume: f32) {
        unsafe { SetMasterVolume(volume) }
    }

    pub(crate) fn __get_master_volume() -> f32 {
        unsafe { GetMasterVolume() }
    }

    // Wave/Sound loading/unloading methods

    pub(crate) fn __load_wave(filename: impl Display) -> Result<Wave, String> {
        unsafe {
            let wave = LoadWave(rl_str!(filename));
            if wave.data.is_null() {
                Err(format!("error loading wave from {}", filename))
            } else {
                Ok(wave)
            }
        }
    }

    pub(crate) fn __load_wave_from_memory(
        tpe: impl Display,
        data: Vec<u8>,
    ) -> Result<Wave, String> {
        unsafe {
            let size = data.len() as i32;
            let wave = LoadWaveFromMemory(rl_str!(tpe), data.as_ptr(), size);
            if wave.data.is_null() {
                Err("error loading wave from memory".to_owned())
            } else {
                Ok(wave)
            }
        }
    }

    pub(crate) fn __is_wave_ready(wave: Wave) -> bool {
        unsafe { IsWaveReady(wave) }
    }

    pub(crate) fn __load_sound(filename: impl Display) -> Result<Sound, String> {
        unsafe {
            let sound = LoadSound(rl_str!(filename));
            if sound.stream.buffer.is_null() {
                Err(format!("error loading sound from {}", filename))
            } else {
                Ok(sound)
            }
        }
    }

    pub(crate) fn __load_sound_from_wave(wave: Wave) -> Sound {
        unsafe { LoadSoundFromWave(wave) }
    }

    pub(crate) fn __load_sound_alias(source: Sound) -> Sound {
        unsafe { LoadSoundAlias(source) }
    }

    pub(crate) fn __is_sound_ready(sound: Sound) -> bool {
        unsafe { IsSoundReady(sound) }
    }

    pub(crate) fn __unload_wave(wave: Wave) {
        unsafe { UnloadWave(wave) }
    }

    pub(crate) fn __unload_sound(sound: Sound) {
        unsafe { UnloadSound(sound) }
    }

    pub(crate) fn __unload_sound_alias(sound: Sound) {
        unsafe { UnloadSoundAlias(sound) }
    }

    pub(crate) fn __export_wave(wave: Wave, filename: impl Display) -> bool {
        unsafe { ExportWave(wave, rl_str!(filename)) }
    }

    pub(crate) fn __export_wave_as_code(wave: Wave, filename: impl Display) -> bool {
        unsafe { ExportWaveAsCode(wave, rl_str!(filename)) }
    }

    // Wave/Sound management methods

    pub(crate) fn __play_sound(sound: Sound) {
        unsafe { PlaySound(sound) }
    }

    pub(crate) fn __stop_sound(sound: Sound) {
        unsafe { StopSound(sound) }
    }

    pub(crate) fn __pause_sound(sound: Sound) {
        unsafe { PauseSound(sound) }
    }

    pub(crate) fn __resume_sound(sound: Sound) {
        unsafe { ResumeSound(sound) }
    }

    pub(crate) fn __is_sound_playing(sound: Sound) -> bool {
        unsafe { IsSoundPlaying(sound) }
    }

    pub(crate) fn __set_sound_volume(sound: Sound, volume: f32) {
        unsafe { SetSoundVolume(sound, volume) }
    }

    pub(crate) fn __set_sound_pitch(sound: Sound, pitch: f32) {
        unsafe { SetSoundPitch(sound, pitch) }
    }

    pub(crate) fn __set_sound_pan(sound: Sound, pan: f32) {
        unsafe { SetSoundPan(sound, pan) }
    }

    pub(crate) fn __wave_copy(wave: Wave) -> Wave {
        unsafe { WaveCopy(wave) }
    }

    pub(crate) fn __wave_crop(wave: &mut Wave, init_sample: i32, final_sample: i32) {
        unsafe { WaveCrop(wave, init_sample, final_sample) }
    }

    pub(crate) fn __wave_format(
        wave: &mut Wave,
        sample_rate: i32,
        sample_size: i32,
        channels: i32,
    ) {
        unsafe { WaveFormat(wave, sample_rate, sample_size, channels) }
    }

    pub(crate) fn __load_wave_samples(wave: Wave) -> Result<Vec<f32>, String> {
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

    pub(crate) fn __load_music_stream(filename: impl Display) -> Result<Music, String> {
        unsafe {
            let music = LoadMusicStream(rl_str!(filename));
            if music.stream.buffer.is_null() {
                Err(format!("failed to load music stream from {}", filename))
            } else {
                Ok(music)
            }
        }
    }

    pub(crate) fn __load_music_stream_from_memory(
        tpe: impl Display,
        data: Vec<u8>,
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

    pub(crate) fn __is_music_ready(music: Music) -> bool {
        unsafe { IsMusicReady(music) }
    }

    pub(crate) fn __unload_music_stream(music: Music) {
        unsafe { UnloadMusicStream(music) }
    }

    pub(crate) fn __play_music_stream(music: Music) {
        unsafe { PlayMusicStream(music) }
    }

    pub(crate) fn __is_music_stream_playing(music: Music) -> bool {
        unsafe { IsMusicStreamPlaying(music) }
    }

    pub(crate) fn __update_music_stream(music: Music) {
        unsafe { UpdateMusicStream(music) }
    }

    pub(crate) fn __stop_music_stream(music: Music) {
        unsafe { StopMusicStream(music) }
    }

    pub(crate) fn __pause_music_stream(music: Music) {
        unsafe { PauseMusicStream(music) }
    }

    pub(crate) fn __resume_music_stream(music: Music) {
        unsafe { ResumeMusicStream(music) }
    }

    pub(crate) fn __seek_music_stream(music: Music, position: f32) {
        unsafe { SeekMusicStream(music, position) }
    }

    pub(crate) fn __set_music_volume(music: Music, volume: f32) {
        unsafe { SetMusicVolume(music, volume) }
    }

    pub(crate) fn __set_music_pitch(music: Music, pitch: f32) {
        unsafe { SetMusicPitch(music, pitch) }
    }

    pub(crate) fn __set_music_pan(music: Music, pan: f32) {
        unsafe { SetMusicPan(music, pan) }
    }

    pub(crate) fn __get_music_time_length(music: Music) -> f32 {
        unsafe { GetMusicTimeLength(music) }
    }

    pub(crate) fn __get_music_time_played(music: Music) -> f32 {
        unsafe { GetMusicTimePlayed(music) }
    }

    // AudioStream management methods

    pub(crate) fn __load_audio_stream(
        sample_rate: u32,
        sample_size: u32,
        channels: u32,
    ) -> Result<AudioStream, String> {
        unsafe {
            // TODO: test whether audio stream was properly loaded
            Ok(LoadAudioStream(sample_rate, sample_size, channels))
        }
    }

    pub(crate) fn __is_audio_stream_ready(stream: AudioStream) -> bool {
        unsafe { IsAudioStreamReady(stream) }
    }

    pub(crate) fn __unload_audio_stream(stream: AudioStream) {
        unsafe { UnloadAudioStream(stream) }
    }

    pub(crate) fn __update_audio_stream(stream: AudioStream, data: Vec<u8>) {
        unsafe {
            let count = data.len() as i32;
            let data = data.as_ptr() as *const c_void;
            UpdateAudioStream(stream, data, count)
        }
    }

    pub(crate) fn __is_audio_stream_processed(stream: AudioStream) -> bool {
        unsafe { IsAudioStreamProcessed(stream) }
    }

    pub(crate) fn __play_audio_stream(stream: AudioStream) {
        unsafe { PlayAudioStream(stream) }
    }

    pub(crate) fn __pause_audio_stream(stream: AudioStream) {
        unsafe { PauseAudioStream(stream) }
    }

    pub(crate) fn __resume_audio_stream(stream: AudioStream) {
        unsafe { ResumeAudioStream(stream) }
    }

    pub(crate) fn __is_audio_stream_playing(stream: AudioStream) -> bool {
        unsafe { IsAudioStreamPlaying(stream) }
    }

    pub(crate) fn __stop_audio_stream(stream: AudioStream) {
        unsafe { StopAudioStream(stream) }
    }

    pub(crate) fn __set_audio_stream_volume(stream: AudioStream, volume: f32) {
        unsafe { SetAudioStreamVolume(stream, volume) }
    }

    pub(crate) fn __set_audio_stream_pitch(stream: AudioStream, pitch: f32) {
        unsafe { SetAudioStreamPitch(stream, pitch) }
    }

    pub(crate) fn __set_audio_stream_pan(stream: AudioStream, pan: f32) {
        unsafe { SetAudioStreamPan(stream, pan) }
    }

    pub(crate) fn __set_audio_stream_buffer_size_default(size: i32) {
        unsafe { SetAudioStreamBufferSizeDefault(size) }
    }

    // TODO: SetAudioStreamCallback
    // TODO: AttachAudioStreamProcessor
    // TODO: DetachAudioStreamProcessor
    // TODO: AttachAudioMixedProcessor
    // TODO: DetachAudioMixedProcessor
}

/// Exported methods
impl Raudio {
    // Audio device management

    /// Initialize audio device and context
    pub fn init_audio_device(&self) {
        Self::__init_audio_device()
    }

    /// Close the audio device and context
    pub fn close_audio_device(&self) {
        Self::__close_audio_device()
    }

    /// Check whether audio device has been initialized successfully
    pub fn is_audio_device_ready(&self) -> bool {
        Self::__is_audio_device_ready()
    }

    /// Set master volume (listener)
    pub fn set_master_volume(&self, volume: f32) {
        Self::__set_master_volume(volume)
    }

    /// Get master volume (listener)
    pub fn get_master_volume(&self) -> f32 {
        Self::__get_master_volume()
    }

    // Wave/Sound loading/unloading methods

    /// Load wave data from file
    pub fn load_wave(&self, filename: impl Display) -> Result<Wave, String> {
        Self::__load_wave(filename)
    }

    /// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
    pub fn load_wave_from_memory(&self, tpe: WaveType, data: Vec<u8>) -> Result<Wave, String> {
        Self::__load_wave_from_memory(tpe, data)
    }

    /// Check whether wave data is ready
    pub fn is_wave_ready(&self, wave: Wave) -> bool {
        Self::__is_wave_ready(wave)
    }

    /// Load sound from file
    pub fn load_sound(&self, filename: impl Display) -> Result<Sound, String> {
        Self::__load_sound(filename)
    }

    /// Load sound from wave data
    pub fn load_sound_from_wave(&self, wave: Wave) -> Sound {
        Self::__load_sound_from_wave(wave)
    }

    /// Create a new sound that shares the same sample data as the source sound, does not own the sound data
    pub fn load_sound_alias(&self, source: Sound) -> Sound {
        Self::__load_sound_alias(source)
    }

    /// Check whether a sound is ready
    pub fn is_sound_ready(&self, sound: Sound) -> bool {
        Self::__is_sound_ready(sound)
    }

    /// Unload wave data
    pub fn unload_wave(&self, wave: Wave) {
        Self::__unload_wave(wave)
    }

    /// Unload sound
    pub fn unload_sound(&self, sound: Sound) {
        Self::__unload_sound(sound)
    }

    /// Unload a sound alias (does not deallocate sample data)
    pub fn unload_sound_alias(&self, sound: Sound) {
        Self::__unload_sound_alias(sound)
    }

    /// Export wave data to file, returns true on success
    pub fn export_wave(&self, wave: Wave, filename: impl Display) -> bool {
        Self::__export_wave(wave, filename)
    }

    /// Export wave sample data to code (.h), returns true on success
    pub fn export_wave_as_code(&self, wave: Wave, filename: impl Display) -> bool {
        Self::__export_wave_as_code(wave, filename)
    }

    // Wave/Sound management methods

    /// Play a sound
    pub fn play_sound(&self, sound: Sound) {
        Self::__play_sound(sound)
    }

    /// Stop playing a sound
    pub fn stop_sound(&self, sound: Sound) {
        Self::__stop_sound(sound)
    }

    /// Pause a sound
    pub fn pause_sound(&self, sound: Sound) {
        Self::__pause_sound(sound)
    }

    /// Resume a paused sound
    pub fn resume_sound(&self, sound: Sound) {
        Self::__resume_sound(sound)
    }

    /// Check whether a sound is currently playing
    pub fn is_sound_playing(&self, sound: Sound) -> bool {
        Self::__is_sound_playing(sound)
    }

    /// Set volume for a sound (1.0 is max level)
    pub fn set_sound_volume(&self, sound: Sound, volume: f32) {
        Self::__set_sound_volume(sound, volume)
    }

    /// Set pitch for a sound (1.0 is base level)
    pub fn set_sound_pitch(&self, sound: Sound, pitch: f32) {
        Self::__set_sound_pitch(sound, pitch)
    }

    /// Set pan for a sound (0.5 is center)
    pub fn set_sound_pan(&self, sound: Sound, pan: f32) {
        Self::__set_sound_pan(sound, pan)
    }

    /// Copy a wave to a new wave
    pub fn wave_copy(&self, wave: Wave) -> Wave {
        Self::__wave_copy(wave)
    }

    // Crop a wave to defined samples range
    pub fn wave_crop(&self, wave: &mut Wave, init_sample: i32, final_sample: i32) {
        Self::__wave_crop(wave, init_sample, final_sample)
    }

    /// Convert wave data to desired format
    pub fn wave_format(&self, wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
        Self::__wave_format(wave, sample_rate, sample_size, channels)
    }

    /// Load samples data from wave as a 32bit float data array
    pub fn load_wave_samples(&self, wave: Wave) -> Result<Vec<f32>, String> {
        Self::__load_wave_samples(wave)
    }

    // Music management methods

    /// Load music stream from file
    pub fn load_music_stream(&self, filename: impl Display) -> Result<Music, String> {
        Self::__load_music_stream(filename)
    }

    /// Load music stream from data
    pub fn load_music_stream_from_memory(
        &self,
        tpe: impl Display,
        data: Vec<u8>,
    ) -> Result<Music, String> {
        Self::__load_music_stream_from_memory(tpe, data)
    }

    /// Check whether a music stream is ready
    pub fn is_music_ready(&self, music: Music) -> bool {
        Self::__is_music_ready(music)
    }

    /// Unload music stream
    pub fn unload_music_stream(&self, music: Music) {
        Self::__unload_music_stream(music)
    }

    /// Start music playing
    pub fn play_music_stream(&self, music: Music) {
        Self::__play_music_stream(music)
    }

    /// Check whether music is playing
    pub fn is_music_stream_playing(&self, music: Music) -> bool {
        Self::__is_music_stream_playing(music)
    }

    /// Updates buffers for music streaming
    pub fn update_music_stream(&self, music: Music) {
        Self::__update_music_stream(music)
    }

    /// Stop music playing
    pub fn stop_music_stream(&self, music: Music) {
        Self::__stop_music_stream(music)
    }

    /// Pause music playing
    pub fn pause_music_stream(&self, music: Music) {
        Self::__pause_music_stream(music)
    }

    /// Resume playing paused music
    pub fn resume_music_stream(&self, music: Music) {
        Self::__resume_music_stream(music)
    }

    /// Seek music to a position (in seconds)
    pub fn seek_music_stream(&self, music: Music, position: f32) {
        Self::__seek_music_stream(music, position)
    }

    /// Set volume for music (1.0 is max level)
    pub fn set_music_volume(&self, music: Music, volume: f32) {
        Self::__set_music_volume(music, volume)
    }

    /// Set pitch for a music (1.0 is base level)
    pub fn set_music_pitch(&self, music: Music, pitch: f32) {
        Self::__set_music_pitch(music, pitch)
    }

    /// Set pan for a music (0.5 is center)
    pub fn set_music_pan(&self, music: Music, pan: f32) {
        Self::__set_music_pan(music, pan)
    }

    /// Get music time length (in seconds)
    pub fn get_music_time_length(&self, music: Music) -> f32 {
        Self::__get_music_time_length(music)
    }

    /// Get current music time played (in seconds)
    pub fn get_music_time_played(&self, music: Music) -> f32 {
        Self::__get_music_time_played(music)
    }

    // AudioStream management methods

    /// Load audio stream (to stream raw audio pcm data)
    pub fn load_audio_stream(
        &self,
        sample_rate: u32,
        sample_size: u32,
        channels: u32,
    ) -> Result<AudioStream, String> {
        Self::__load_audio_stream(sample_rate, sample_size, channels)
    }

    /// Check whether an audio stream is ready
    pub fn is_audio_stream_ready(&self, stream: AudioStream) -> bool {
        Self::__is_audio_stream_ready(stream)
    }

    /// Unload audio stream and free memory
    pub fn unload_audio_stream(&self, stream: AudioStream) {
        Self::__unload_audio_stream(stream)
    }

    /// Update audio stream buffers with data
    pub fn update_audio_stream(&self, stream: AudioStream, data: Vec<u8>) {
        Self::__update_audio_stream(stream, data)
    }

    /// Check whether any audio stream buffers requires refill
    pub fn is_audio_stream_processed(&self, stream: AudioStream) -> bool {
        Self::__is_audio_stream_processed(stream)
    }

    /// Play audio stream
    pub fn play_audio_stream(&self, stream: AudioStream) {
        Self::__play_audio_stream(stream)
    }

    /// Pause audio stream
    pub fn pause_audio_stream(&self, stream: AudioStream) {
        Self::__pause_audio_stream(stream)
    }

    /// Resume audio stream
    pub fn resume_audio_stream(&self, stream: AudioStream) {
        Self::__resume_audio_stream(stream)
    }

    /// Check whether audio stream is playing
    pub fn is_audio_stream_playing(&self, stream: AudioStream) -> bool {
        Self::__is_audio_stream_playing(stream)
    }

    /// Stop audio stream
    pub fn stop_audio_stream(&self, stream: AudioStream) {
        Self::__stop_audio_stream(stream)
    }

    /// Set volume for audio stream (1.0 is max level)
    pub fn set_audio_stream_volume(&self, stream: AudioStream, volume: f32) {
        Self::__set_audio_stream_volume(stream, volume)
    }

    /// Set pitch for audio stream (1.0 is base level)
    pub fn set_audio_stream_pitch(&self, stream: AudioStream, pitch: f32) {
        Self::__set_audio_stream_pitch(stream, pitch)
    }

    /// Set pan for audio stream (0.5 is centered)
    pub fn set_audio_stream_pan(&self, stream: AudioStream, pan: f32) {
        Self::__set_audio_stream_pan(stream, pan)
    }

    /// Default size for new audio streams
    pub fn set_audio_stream_buffer_size_default(&self, size: i32) {
        Self::__set_audio_stream_buffer_size_default(size)
    }
}
