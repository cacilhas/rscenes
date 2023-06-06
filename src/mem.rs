use std::ffi::CString;

use raylib::prelude::*;

// TODO: identify other *_from_mem functions missing in raylib 3.7

/// Returns a Wave object from memory raw data.
/// `filetype` must be the type extension, including the dot.
/// For instance: `".wav"`
pub fn load_wave(filetype: &'static str, bytes: &'static [u8]) -> Result<Wave, String> {
    let bytes = bytes.iter().map(|e| e.to_owned()).collect::<Vec<u8>>();
    unsafe { load_wave_from_mem(filetype, &bytes, bytes.len() as i32) }
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
