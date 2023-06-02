use std::{cell::RefCell, rc::Rc};

use crate::status::Status;
use raylib::prelude::*;

pub trait Scene {
    #[allow(unused_variables)]
    fn init(&mut self, handle: (&mut RaylibHandle, &RaylibThread)) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(
        &mut self,
        handle: (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        audio: Option<Rc<RefCell<RaylibAudio>>>,
    ) -> anyhow::Result<Status>;

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        font: Option<Rc<Font>>,
        audio: Option<Rc<RefCell<RaylibAudio>>>,
    ) -> anyhow::Result<()>;
}
