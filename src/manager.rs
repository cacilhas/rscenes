use std::{collections::HashMap, fmt, rc::Rc};

use raylib::prelude::*;

use crate::{scene::Scene, state::State};

/// SceneManager manages multiple scenes.
#[derive(Debug)]
pub struct SceneManager {
    handle: (RaylibHandle, RaylibThread),
    scenes: Vec<Box<dyn Scene>>,
    fonts: HashMap<String, Rc<Font>>,
    audio: Option<RaylibAudio>,
}

impl SceneManager {
    /// Creates a new SceneManager from a RaylibBuilder, which ownership must be taken.
    #[must_use]
    pub fn new(builder: RaylibBuilder) -> Self {
        let (mut handle, thread) = builder.build();
        handle.set_target_fps(60);
        handle.set_exit_key(None);
        Self {
            handle: (handle, thread),
            scenes: Vec::with_capacity(4),
            fonts: HashMap::with_capacity(2),
            audio: None,
        }
    }

    /// Allows to reconfigure the inner RaylibHandle.
    pub fn config<T>(&mut self, callback: impl Fn(&mut RaylibHandle, &RaylibThread) -> T) -> T {
        callback(&mut self.handle.0, &self.handle.1)
    }

    /// Sets the font passed to scenes.
    pub fn push_font(&mut self, name: &str, font: &Rc<Font>) {
        self.fonts.insert(name.to_owned(), font.clone());
    }

    /// Enables audio by initialising the default audio device.
    pub fn enable_audio(&mut self) {
        self.audio = Some(RaylibAudio::init_audio_device());
    }

    /// Adds the first scene to the stack.
    pub fn add_first_scene(&mut self, scene: Box<dyn Scene>) {
        if !self.scenes.is_empty() {
            self.scenes.clear();
        }
        self.scenes.push(scene);
    }

    /// Starts Raylib main loop.
    pub fn start(&mut self) -> anyhow::Result<()> {
        let (handle, thread) = (&mut self.handle.0, &self.handle.1);
        let audio = self.audio.as_mut().map(|a| Rc::new(a));

        match self.scenes.last_mut() {
            Some(scene) => scene.init(handle, thread)?,
            None => return Err(NoSceneLoaded.into()),
        }

        'mainloop: while !handle.window_should_close() {
            let screen = Rectangle {
                width: handle.get_screen_width() as f32,
                height: handle.get_screen_height() as f32,
                ..Default::default()
            };

            let state = {
                let state = if handle.is_key_released(KeyboardKey::KEY_ESCAPE) {
                    let _ = handle.begin_drawing(thread);
                    State::Previous(1)
                } else {
                    let scene = match self.scenes.last_mut() {
                        Some(scene) => scene,
                        None => break 'mainloop,
                    };
                    let dt = handle.get_frame_time();
                    let state = match scene.update((handle, thread), dt, audio.clone())? {
                        State::Keep => {
                            let mut fonts =
                                HashMap::<String, Rc<Font>>::with_capacity(self.fonts.len());
                            for key in self.fonts.keys() {
                                fonts.insert(key.to_owned(), self.fonts[key].clone());
                            }
                            let mut handle = handle.begin_drawing(thread);
                            let _ = scene.draw(&mut handle, screen, fonts, audio.clone());
                            State::Keep
                        }
                        status => status,
                    };
                    state
                };
                state
            };

            match state {
                State::New(mut scene) => {
                    scene.init(handle, thread)?;
                    self.scenes.push(scene);
                }

                State::Previous(prev) => {
                    for _ in 0..prev {
                        if let None = self.scenes.pop() {
                            break 'mainloop;
                        }
                    }
                    if let Some(scene) = self.scenes.last_mut() {
                        scene.init(handle, thread)?;
                    }
                }

                _ => (),
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
struct NoSceneLoaded;

impl fmt::Display for NoSceneLoaded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no scene loaded")
    }
}

// TODO: how to test multiple Raylib instances?
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use raylib::{consts::TraceLogLevel, logging::set_trace_log};
//
//     #[test]
//     fn manager_should_start_empty() {
//         set_trace_log(TraceLogLevel::LOG_ERROR);
//         let builder = raylib::init();
//         let manager = SceneManager::new(builder);
//         assert!(manager.scenes.is_empty());
//         assert!(manager.font.is_none());
//         assert!(manager.audio.is_none());
//     }
//
//     #[test]
//     fn default_fps_should_be_60fps() {
//         set_trace_log(TraceLogLevel::LOG_ERROR);
//         let builder = raylib::init();
//         let manager = SceneManager::new(builder);
//         manager.config(|handle, _| {
//             assert_eq!(60, handle.get_fps());
//         });
//     }
// }
