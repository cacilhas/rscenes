use crate::rtext::RtextImpl;
use std::{fmt::Display, marker::PhantomData};

#[derive(Clone, Copy, Debug)]
pub struct Codepoints {
    pub inner: *mut i32,
    pub count: usize,
    phantom: PhantomData<*mut i32>,
}

impl Codepoints {
    /// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
    pub fn load(text: impl Display) -> Result<Self, String> {
        RtextImpl::__load_codepoints(text)
    }

    /// Create a new codepoint set from a C resource
    pub(crate) fn new(ptr: *mut i32, count: i32) -> Self {
        Self {
            inner: ptr,
            count: count as usize,
            phantom: PhantomData,
        }
    }

    /// Unload codepoints data from memory
    pub fn unload(self) {
        RtextImpl::__unload_codepoints(self)
    }

    /// Return amount of codepoints
    pub fn len(&self) -> usize {
        self.count
    }

    /// Return whether the set is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get a codepoint from a specific index
    pub fn get(&self, index: usize) -> Result<i32, String> {
        if self.inner.is_null() {
            return Err("null codepoints".to_owned());
        }
        if index >= self.count {
            return Err("index overflow".to_owned());
        }

        unsafe { Ok(*self.inner.add(index)) }
    }
}

impl Default for Codepoints {
    fn default() -> Self {
        Self {
            // TODO: try ptr::null_mut()
            inner: (32..256).collect::<Vec<_>>().as_mut_ptr(),
            count: 224,
            phantom: PhantomData,
        }
    }
}

impl From<Codepoints> for *mut i32 {
    fn from(val: Codepoints) -> Self {
        val.inner
    }
}

impl From<Codepoints> for *const i32 {
    fn from(val: Codepoints) -> Self {
        val.inner
    }
}
