use std::fmt;

use crate::state::State;
use raylib::prelude::*;

#[cfg(feature = "eyre")]
use eyre as anyhow;

/// Trait to make scenes usable.
pub trait Scene<R = ()>: fmt::Debug {
    /// Initialises the scene from RaylibHandle.
    #[allow(unused_variables)]
    fn init(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> anyhow::Result<()> {
        Ok(())
    }

    /// Updates the scene each frame.
    #[must_use]
    fn update(
        &mut self,
        rl: (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        resources: &mut R,
    ) -> anyhow::Result<State<R>>;

    /// Draws the scene each frame.
    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        resources: &R,
    ) -> anyhow::Result<()>;
}
