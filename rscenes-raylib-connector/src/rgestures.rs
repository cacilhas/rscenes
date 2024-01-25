use raylib_ffi::{enums::Gesture, *};

#[derive(Clone, Copy, Debug)]
pub struct Rgestures;

/// Crate only methods
impl Rgestures {
    pub(crate) fn __set_gestures_enable(flags: impl Into<usize>) {
        unsafe { SetGesturesEnabled(flags.into() as u32) }
    }

    pub(crate) fn __is_gesture_detected(gesture: impl Into<usize>) -> bool {
        unsafe { IsGestureDetected(gesture.into() as u32) }
    }

    pub(crate) fn __get_gesture_detected() -> usize {
        unsafe { GetGestureDetected() as usize }
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
    pub fn set_gestures_enable(&self, flags: impl Into<usize>) {
        Self::__set_gestures_enable(flags)
    }

    pub fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        Self::__is_gesture_detected(gesture)
    }

    pub fn get_gesture_detected(&self) -> usize {
        Self::__get_gesture_detected()
    }

    pub fn get_gesture_hold_duration(&self) -> f32 {
        Self::__get_gesture_hold_duration()
    }

    pub fn get_gesture_drag_vector(&self) -> Vector2 {
        Self::__get_gesture_drag_vector()
    }

    pub fn get_gesture_drag_angle(&self) -> f32 {
        Self::__get_gesture_drag_angle()
    }

    pub fn get_gesture_pinch_vector(&self) -> Vector2 {
        Self::__get_gesture_pinch_vector()
    }

    pub fn get_gesture_pinch_angle(&self) -> f32 {
        Self::__get_gesture_pinch_angle()
    }
}
