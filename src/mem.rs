use std::ffi::CString;

use raylib::prelude::*;

// TODO: identify other *_from_mem functions missing in raylib 3.7

/// Returns a Wave object from memory raw data.
/// For `filetype`, use `mem::WaveType`.
pub fn load_wave(filetype: impl Into<&'static str>, bytes: &'static [u8]) -> Result<Wave, String> {
    let bytes = bytes.iter().map(|e| e.to_owned()).collect::<Vec<u8>>();
    unsafe { load_wave_from_mem(filetype.into(), &bytes, bytes.len() as i32) }
}

unsafe fn load_wave_from_mem(filetype: &str, bytes: &Vec<u8>, size: i32) -> Result<Wave, String> {
    let c_filetype = CString::new(filetype).unwrap();
    let c_bytes = bytes.as_ptr();
    let w = ffi::LoadWaveFromMemory(c_filetype.as_ptr(), c_bytes, size);
    if w.data.is_null() {
        return Err(format!("Wave data is null. Check provided buffer data"));
    };
    Ok(Wave::from_raw(w))
}

#[derive(Clone, Copy, Debug)]
pub enum WaveType {
    Aac,
    Ape,
    Au,
    Flat,
    Mp3,
    M4a,
    Ogg,
    Opus,
    Qoa,
    Wav,
}

impl From<WaveType> for &'static str {
    fn from(value: WaveType) -> Self {
        match value {
            WaveType::Aac => ".aac",
            WaveType::Ape => ".ape",
            WaveType::Au => ".au",
            WaveType::Flat => ".flat",
            WaveType::Mp3 => ".mp3",
            WaveType::M4a => ".m4a",
            WaveType::Ogg => ".ogg",
            WaveType::Opus => ".opus",
            WaveType::Qoa => ".qoa",
            WaveType::Wav => ".wav",
        }
    }
}
