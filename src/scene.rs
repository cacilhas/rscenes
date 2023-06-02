use std::{fmt, rc::Rc};

use crate::state::State;
use raylib::prelude::*;

pub trait Scene: fmt::Debug {
    #[allow(unused_variables)]
    fn init(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(
        &mut self,
        rl: (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        audio: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<State>;

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        font: Option<Rc<Font>>,
        audio: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<()>;
}
