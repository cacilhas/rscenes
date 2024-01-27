use raylib_ffi::*;
use std::fmt::Display;

use crate::utils::array_from_c;

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
            array_from_c(raw, count as usize, || {
                "couldn't load samples from wave".to_owned()
            })
        }
    }

    pub(crate) fn __unload_wave_samples(samples: &mut Vec<f32>) {
        unsafe { UnloadWaveSamples(samples.as_mut_ptr()) }
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
}

/// Exported methods
impl Raudio {
    // Audio device management

    pub fn init_audio_device(&self) {
        Self::__init_audio_device()
    }

    pub fn close_audio_device(&self) {
        Self::__close_audio_device()
    }

    pub fn is_audio_device_ready(&self) -> bool {
        Self::__is_audio_device_ready()
    }

    pub fn set_master_volume(&self, volume: f32) {
        Self::__set_master_volume(volume)
    }

    pub fn get_master_volume(&self) -> f32 {
        Self::__get_master_volume()
    }

    // Wave/Sound loading/unloading methods

    pub fn load_wave(&self, filename: impl Display) -> Result<Wave, String> {
        Self::__load_wave(filename)
    }

    pub fn load_wave_from_memory(&self, tpe: impl Display, data: Vec<u8>) -> Result<Wave, String> {
        Self::__load_wave_from_memory(tpe, data)
    }

    pub fn is_wave_ready(&self, wave: Wave) -> bool {
        Self::__is_wave_ready(wave)
    }

    pub fn load_sound(&self, filename: impl Display) -> Result<Sound, String> {
        Self::__load_sound(filename)
    }

    pub fn load_sound_from_wave(&self, wave: Wave) -> Sound {
        Self::__load_sound_from_wave(wave)
    }

    pub fn load_sound_alias(&self, source: Sound) -> Sound {
        Self::__load_sound_alias(source)
    }

    pub fn is_sound_ready(&self, sound: Sound) -> bool {
        Self::__is_sound_ready(sound)
    }

    pub fn unload_wave(&self, wave: Wave) {
        Self::__unload_wave(wave)
    }

    pub fn unload_sound(&self, sound: Sound) {
        Self::__unload_sound(sound)
    }

    pub fn unload_sound_alias(&self, sound: Sound) {
        Self::__unload_sound_alias(sound)
    }

    pub fn export_wave(&self, wave: Wave, filename: impl Display) -> bool {
        Self::__export_wave(wave, filename)
    }

    pub fn export_wave_as_code(&self, wave: Wave, filename: impl Display) -> bool {
        Self::__export_wave_as_code(wave, filename)
    }

    // Wave/Sound management methods

    pub fn play_sound(&self, sound: Sound) {
        Self::__play_sound(sound)
    }

    pub fn stop_sound(&self, sound: Sound) {
        Self::__stop_sound(sound)
    }

    pub fn pause_sound(&self, sound: Sound) {
        Self::__pause_sound(sound)
    }

    pub fn resume_sound(&self, sound: Sound) {
        Self::__resume_sound(sound)
    }

    pub fn is_sound_playing(&self, sound: Sound) -> bool {
        Self::__is_sound_playing(sound)
    }

    pub fn set_sound_volume(&self, sound: Sound, volume: f32) {
        Self::__set_sound_volume(sound, volume)
    }

    pub fn set_sound_pitch(&self, sound: Sound, pitch: f32) {
        Self::__set_sound_pitch(sound, pitch)
    }

    pub fn set_sound_pan(&self, sound: Sound, pan: f32) {
        Self::__set_sound_pan(sound, pan)
    }

    pub fn wave_copy(&self, wave: Wave) -> Wave {
        Self::__wave_copy(wave)
    }

    pub fn wave_crop(&self, wave: &mut Wave, init_sample: i32, final_sample: i32) {
        Self::__wave_crop(wave, init_sample, final_sample)
    }

    pub fn wave_format(&self, wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
        Self::__wave_format(wave, sample_rate, sample_size, channels)
    }

    pub fn load_wave_samples(&self, wave: Wave) -> Result<Vec<f32>, String> {
        Self::__load_wave_samples(wave)
    }

    pub fn unload_wave_samples(&self, samples: &mut Vec<f32>) {
        Self::__unload_wave_samples(samples)
    }

    // Music management methods

    pub fn load_music_stream(&self, filename: impl Display) -> Result<Music, String> {
        Self::__load_music_stream(filename)
    }

    pub fn load_music_stream_from_memory(
        &self,
        tpe: impl Display,
        data: Vec<u8>,
    ) -> Result<Music, String> {
        Self::__load_music_stream_from_memory(tpe, data)
    }

    pub fn is_music_ready(&self, music: Music) -> bool {
        Self::__is_music_ready(music)
    }

    pub fn unload_music_stream(&self, music: Music) {
        Self::__unload_music_stream(music)
    }

    pub fn play_music_stream(&self, music: Music) {
        Self::__play_music_stream(music)
    }

    pub fn is_music_stream_playing(&self, music: Music) -> bool {
        Self::__is_music_stream_playing(music)
    }

    pub fn update_music_stream(&self, music: Music) {
        Self::__update_music_stream(music)
    }

    pub fn stop_music_stream(&self, music: Music) {
        Self::__stop_music_stream(music)
    }

    pub fn pause_music_stream(&self, music: Music) {
        Self::__pause_music_stream(music)
    }

    pub fn resume_music_stream(&self, music: Music) {
        Self::__resume_music_stream(music)
    }

    pub fn seek_music_stream(&self, music: Music, position: f32) {
        Self::__seek_music_stream(music, position)
    }

    pub fn set_music_volume(&self, music: Music, volume: f32) {
        Self::__set_music_volume(music, volume)
    }

    pub fn set_music_pitch(&self, music: Music, pitch: f32) {
        Self::__set_music_pitch(music, pitch)
    }

    pub fn set_music_pan(&self, music: Music, pan: f32) {
        Self::__set_music_pan(music, pan)
    }

    pub fn get_music_time_length(&self, music: Music) -> f32 {
        Self::__get_music_time_length(music)
    }

    pub fn get_music_time_played(&self, music: Music) -> f32 {
        Self::__get_music_time_played(music)
    }

    // AudioStream management methods
}
