use crate::rtext::Rtext;
use std::{fmt::Display, marker::PhantomData, ptr};

#[derive(Clone, Copy, Debug)]
pub struct Codepoints {
    pub inner: *mut i32,
    pub count: usize,
    phantom: PhantomData<*mut i32>,
}

impl Codepoints {
    pub fn load(text: impl Display) -> Result<Self, String> {
        Rtext::__load_codepoints(text)
    }

    pub(crate) fn new(ptr: *mut i32, count: i32) -> Self {
        Self {
            inner: ptr,
            count: count as usize,
            phantom: PhantomData,
        }
    }

    pub fn unload(self) {
        Rtext::__unload_codepoints(self)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn get(&self, index: usize) -> Result<i32, String> {
        if self.inner == ptr::null_mut() {
            return Err("null codepoints".to_owned());
        }
        if index >= self.count {
            return Err("index overflow".to_owned());
        }

        unsafe { Ok(*self.inner.add(index)) }
    }
}

impl Into<*mut i32> for Codepoints {
    fn into(self) -> *mut i32 {
        self.inner
    }
}

impl Into<*const i32> for Codepoints {
    fn into(self) -> *const i32 {
        self.inner
    }
}
