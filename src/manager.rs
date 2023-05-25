use std::{cell::RefCell, rc::Rc};

use chrono::Utc;
use raylib::prelude::*;

use crate::{Scene, State};

macro_rules! handle_thr {
    ($self:expr) => {
        (&mut $self.handle, &$self.thread)
    };
}

macro_rules! scene_init {
    ($scene: expr, $wrap: expr) => {
        $scene.init(
            handle_thr!($wrap),
            $wrap.screen,
            $wrap.font.clone(),
            $wrap.audio.clone(),
        )
    };
}

pub struct SceneManager {
    handle: RaylibHandle,
    thread: RaylibThread,
    screen: Rectangle,
    font: Option<Rc<Font>>,
    audio: Option<Rc<RefCell<RaylibAudio>>>,
    scenes: Vec<Rc<RefCell<dyn Scene>>>,
}

impl SceneManager {
    pub fn new(
        handle: RaylibHandle,
        thread: RaylibThread,
        screen: Rectangle,
        font: Option<Font>,
        audio: Option<RaylibAudio>,
    ) -> Self {
        Self {
            handle,
            thread,
            screen,
            font: font.map(|font| Rc::new(font)),
            audio: audio.map(|audio| Rc::new(RefCell::new(audio))),
            scenes: vec![],
        }
    }

    pub fn initial_scene(&mut self, scene: Rc<RefCell<dyn Scene>>) -> anyhow::Result<()> {
        // It cannot take ownership of the scene parameter ’cause its size isn’t
        // known at compiling time
        scene_init!(scene.borrow_mut(), self)?;
        self.scenes.push(scene);
        Ok(())
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let mut tick = Utc::now();

        'mainloop: while !self.handle.window_should_close() {
            let new_tick = Utc::now();
            let state = {
                let mut draw = self.handle.begin_drawing(&self.thread);
                if draw.is_key_released(KeyboardKey::KEY_ESCAPE) {
                    self.scenes.pop();
                }
                let scene = match self.scenes.last() {
                    Some(scene) => scene,
                    None => break 'mainloop,
                };
                scene.borrow_mut().update(&mut draw, new_tick - tick)?
            };
            match state {
                State::New(scene) => scene_init!(scene.borrow_mut(), self)?,
                State::Previous(pop) => {
                    for _ in 0..pop {
                        if self.scenes.pop().is_none() {
                            break 'mainloop;
                        }
                    }
                }
                State::Keep => (),
            }
            tick = new_tick;
        }
        Ok(())
    }
}
