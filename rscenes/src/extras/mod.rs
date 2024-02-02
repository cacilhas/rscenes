#[cfg(feature = "fake-fs")]
mod fake_fullscreen;
#[cfg(feature = "storage")]
mod store;

#[cfg(feature = "fake-fs")]
pub use fake_fullscreen::FakeFullscreen;
#[cfg(feature = "storage")]
pub use store::XDGStore;
