use std::{ffi::c_void, marker::PhantomData};

#[derive(Debug)]
pub struct WindowHandle<'a> {
    handle: *mut c_void,
    data: PhantomData<&'a c_void>,
}

impl From<*mut c_void> for WindowHandle<'_> {
    fn from(handle: *mut c_void) -> Self {
        WindowHandle {
            handle,
            data: PhantomData,
        }
    }
}

impl Into<*mut c_void> for WindowHandle<'_> {
    fn into(self) -> *mut c_void {
        self.handle
    }
}
