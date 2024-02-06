#[cfg(feature = "fake-fs")]
mod fake_fullscreen;
#[cfg(feature = "lock-mouse")]
mod lock_mouse;
#[cfg(feature = "storage")]
mod store;

#[cfg(feature = "fake-fs")]
pub use fake_fullscreen::{start_fullscreen, FakeFullscreen};
#[cfg(feature = "lock-mouse")]
pub use lock_mouse::LockMouse;
#[cfg(feature = "storage")]
pub use store::XDGStore;
