mod manager;
mod state;

use std::{cell::RefCell, rc::Rc};

pub use crate::manager::SceneManager;
pub use crate::state::State;
use chrono::Duration;
use raylib::prelude::*;

pub trait Scene {
    #[allow(unused_variables)]
    fn init(
        &mut self,
        handle: (&mut RaylibHandle, &RaylibThread),
        screen: Rectangle,
        font: Option<Rc<Font>>,
        audio: Option<Rc<RefCell<RaylibAudio>>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&mut self, draw: &mut RaylibDrawHandle, dt: Duration) -> anyhow::Result<State>;
}
