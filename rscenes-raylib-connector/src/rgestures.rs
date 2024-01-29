use raylib_ffi::{enums::Gesture, *};
use std::{fmt::Debug, usize};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RgesturesImpl;

/// Crate only methods
impl RgesturesImpl {
    pub fn __set_gestures_enabled(flags: impl Into<usize>) {
        unsafe { SetGesturesEnabled(flags.into() as u32) }
    }

    pub fn __is_gesture_detected(gesture: impl Into<usize>) -> bool {
        unsafe { IsGestureDetected(gesture.into() as u32) }
    }

    pub fn __get_gesture_detected() -> Vec<Gesture> {
        unsafe {
            let raw = GetGestureDetected() as usize;
            let mut res: Vec<Gesture> = vec![];
            for &gesture in vec![
                Gesture::Tap,
                Gesture::Doubletap,
                Gesture::Hold,
                Gesture::Drag,
                Gesture::SwipeRight,
                Gesture::SwipeLeft,
                Gesture::SwipeUp,
                Gesture::SwipeDown,
                Gesture::PinchIn,
                Gesture::PinchOut,
            ]
            .iter()
            {
                let code: usize = gesture.into();
                if raw & code != 0 {
                    res.push(gesture);
                }
            }
            res
        }
    }

    pub fn __get_gesture_hold_duration() -> f32 {
        unsafe { GetGestureHoldDuration() }
    }

    pub fn __get_gesture_drag_vector() -> Vector2 {
        unsafe { GetGestureDragVector() }
    }

    pub fn __get_gesture_drag_angle() -> f32 {
        unsafe { GetGestureDragAngle() }
    }

    pub fn __get_gesture_pinch_vector() -> Vector2 {
        unsafe { GetGesturePinchVector() }
    }

    pub fn __get_gesture_pinch_angle() -> f32 {
        unsafe { GetGesturePinchAngle() }
    }
}

/// Exported methods
pub trait Rgestures: Debug {
    // Enable a set of gestures using flags
    fn set_gestures_enable(&self, flags: impl Into<usize>) {
        RgesturesImpl::__set_gestures_enabled(flags)
    }

    /// Check whether a gesture have been detected
    fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        RgesturesImpl::__is_gesture_detected(gesture)
    }

    /// Get latest detected gesture
    fn get_gesture_detected(&self) -> Vec<Gesture> {
        RgesturesImpl::__get_gesture_detected()
    }

    /// Get gesture hold time in milliseconds
    fn get_gesture_hold_duration(&self) -> f32 {
        RgesturesImpl::__get_gesture_hold_duration()
    }

    /// Get gesture drag vector
    fn get_gesture_drag_vector(&self) -> Vector2 {
        RgesturesImpl::__get_gesture_drag_vector()
    }

    /// Get gesture drag angle
    fn get_gesture_drag_angle(&self) -> f32 {
        RgesturesImpl::__get_gesture_drag_angle()
    }

    /// Get gesture pinch delta
    fn get_gesture_pinch_vector(&self) -> Vector2 {
        RgesturesImpl::__get_gesture_pinch_vector()
    }

    /// Get gesture pinch angle
    fn get_gesture_pinch_angle(&self) -> f32 {
        RgesturesImpl::__get_gesture_pinch_angle()
    }
}
