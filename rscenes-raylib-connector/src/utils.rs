use std::{
    ffi::{c_char, CString},
    ptr, slice,
};

pub unsafe fn string_from_c(raw: *mut c_char) -> Result<String, String> {
    CString::from_raw(raw)
        .into_string()
        .or_else(|e| Err(format!("{}", e)))
}

pub unsafe fn utf8_from_c(raw: *const u8, size: usize) -> Result<String, String> {
    let bytes = slice::from_raw_parts(raw, size);
    std::str::from_utf8(bytes)
        .map(|s| s.to_owned())
        .or_else(|e| Err(format!("{}", e)))
}

pub unsafe fn array_from_c<T>(
    raw: *mut T,
    size: usize,
    error: impl FnOnce() -> String,
) -> Result<Vec<T>, String>
where
    T: Clone,
{
    if raw.is_null() {
        Err(error())
    } else {
        let array = ptr::slice_from_raw_parts_mut(raw, size);
        Ok((*array).to_vec())
    }
}
