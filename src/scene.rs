use std::{collections::HashMap, fmt, rc::Rc};

use crate::state::State;
use raylib::prelude::*;

/// Trait to make scenes usable.
pub trait Scene: fmt::Debug {
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
        audio: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<State>;

    /// Draws the scene each frame.
    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        font: HashMap<&'static str, Rc<Font>>,
        audio: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<()>;
}
