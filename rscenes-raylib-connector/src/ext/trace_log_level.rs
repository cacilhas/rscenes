use crate::rcore::RcoreImpl;
use raylib_ffi::enums::TraceLogLevel;
use std::fmt::Display;

pub trait TraceLogLevelExt: Sized {
    fn log(self, text: impl Display);
    fn set_default(self);
}

impl TraceLogLevelExt for TraceLogLevel {
    fn log(self, text: impl Display) {
        RcoreImpl::__trace_log(self, text)
    }

    fn set_default(self) {
        RcoreImpl::__set_trace_log_level(self)
    }
}
