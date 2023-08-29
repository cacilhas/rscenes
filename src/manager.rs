use std::fmt;

use raylib::prelude::*;

use crate::{scene::Scene, state::State};

#[cfg(feature = "eyre")]
use eyre as anyhow;

/// SceneManager manages multiple scenes.
#[derive(Debug)]
pub struct SceneManager<R = ()> {
    handle: (RaylibHandle, RaylibThread),
    scenes: Vec<Box<dyn Scene<R>>>,
    resources: R,
}

impl<R> SceneManager<R> {
    /// Creates a new SceneManager from a RaylibBuilder, which ownership must be taken.
    /// `resources` can be anything you’d like to share between scenes, like fonts and audio
    /// devices.
    #[must_use]
    pub fn new(builder: RaylibBuilder, resources: R) -> Self {
        let (mut handle, thread) = builder.build();
        handle.set_target_fps(60);
        handle.set_exit_key(None);
        Self {
            handle: (handle, thread),
            scenes: Vec::with_capacity(4),
            resources,
        }
    }

    /// Allows to reconfigure the inner RaylibHandle.
    ///
    /// Callback parameters are the Raylib handle, the Raylib thread, and the resources supplied to
    /// ::new().
    pub fn config<T>(
        &mut self,
        callback: impl Fn(&mut RaylibHandle, &RaylibThread, &mut R) -> T,
    ) -> T {
        callback(&mut self.handle.0, &self.handle.1, &mut self.resources)
    }

    /// Adds the first scene to the stack.
    pub fn add_first_scene(&mut self, scene: Box<dyn Scene<R>>) {
        if !self.scenes.is_empty() {
            self.scenes.clear();
        }
        self.scenes.push(scene);
    }

    /// Starts Raylib main loop.
    pub fn start(&mut self) -> anyhow::Result<()> {
        let (handle, thread) = (&mut self.handle.0, &self.handle.1);

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
            let scene = match self.scenes.last_mut() {
                Some(scene) => scene,
                None => break 'mainloop,
            };

            let state = {
                let state = if handle.is_key_released(KeyboardKey::KEY_ESCAPE) {
                    let _ = handle.begin_drawing(thread);
                    State::Previous(1)
                } else {
                    let dt = handle.get_frame_time();
                    scene.update((handle, thread), dt, &mut self.resources)?
                };
                state
            };

            match state {
                State::Keep => {
                    let mut handle = handle.begin_drawing(thread);
                    scene.draw(&mut handle, screen, &self.resources)?;
                }

                State::New::<R>(mut scene) => {
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
