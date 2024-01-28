use std::usize;

use raylib_ffi::{enums::Gesture, *};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rgestures;

/// Crate only methods
impl Rgestures {
    pub(crate) fn __set_gestures_enabled(flags: impl Into<usize>) {
        unsafe { SetGesturesEnabled(flags.into() as u32) }
    }

    pub(crate) fn __is_gesture_detected(gesture: impl Into<usize>) -> bool {
        unsafe { IsGestureDetected(gesture.into() as u32) }
    }

    pub(crate) fn __get_gesture_detected() -> Vec<Gesture> {
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

    pub(crate) fn __get_gesture_hold_duration() -> f32 {
        unsafe { GetGestureHoldDuration() }
    }

    pub(crate) fn __get_gesture_drag_vector() -> Vector2 {
        unsafe { GetGestureDragVector() }
    }

    pub(crate) fn __get_gesture_drag_angle() -> f32 {
        unsafe { GetGestureDragAngle() }
    }

    pub(crate) fn __get_gesture_pinch_vector() -> Vector2 {
        unsafe { GetGesturePinchVector() }
    }

    pub(crate) fn __get_gesture_pinch_angle() -> f32 {
        unsafe { GetGesturePinchAngle() }
    }
}

/// Exported methods
impl Rgestures {
    // Enable a set of gestures using flags
    pub fn set_gestures_enable(&self, flags: impl Into<usize>) {
        Self::__set_gestures_enabled(flags)
    }

    /// Check whether a gesture have been detected
    pub fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        Self::__is_gesture_detected(gesture)
    }

    /// Get latest detected gesture
    pub fn get_gesture_detected(&self) -> Vec<Gesture> {
        Self::__get_gesture_detected()
    }

    /// Get gesture hold time in milliseconds
    pub fn get_gesture_hold_duration(&self) -> f32 {
        Self::__get_gesture_hold_duration()
    }

    /// Get gesture drag vector
    pub fn get_gesture_drag_vector(&self) -> Vector2 {
        Self::__get_gesture_drag_vector()
    }

    /// Get gesture drag angle
    pub fn get_gesture_drag_angle(&self) -> f32 {
        Self::__get_gesture_drag_angle()
    }

    /// Get gesture pinch delta
    pub fn get_gesture_pinch_vector(&self) -> Vector2 {
        Self::__get_gesture_pinch_vector()
    }

    /// Get gesture pinch angle
    pub fn get_gesture_pinch_angle(&self) -> f32 {
        Self::__get_gesture_pinch_angle()
    }
}
