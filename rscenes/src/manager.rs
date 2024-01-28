use crate::{
    connectors::{Connector2D, Connector3D},
    scene::Scene,
    state::State,
};
use resolution::current_resolution;
use rscenes_raylib_connector::{assets::*, interface::*};

/// Control the game
pub struct Rscenes {
    /// Change this to set the window title
    pub title: String,
    /// change this to set window startup geometry
    pub window_size: (i32, i32),
    setups: Vec<Box<dyn SetupCallback>>,
    scenes: Vec<Box<dyn Scene>>,
}

impl Rscenes {
    /// Add a callback to run on the initialisation
    pub fn add_setup(&mut self, callback: impl SetupCallback) {
        self.setups.push(Box::new(callback));
    }

    /// Add the initial scene
    pub fn set_init(&mut self, scene: Box<dyn Scene>) {
        self.scenes.push(scene);
    }

    /// Start mainloop
    pub fn start(&mut self) -> Result<(), String> {
        if self.scenes.is_empty() {
            return Err("no initial scene supplied".to_owned());
        }

        let connector = RaylibConnector::default();
        let (rcore, rgestures, rcamera) = match connector {
            RaylibConnector {
                rcore,
                rgestures,
                rcamera,
                ..
            } => (rcore, rgestures, rcamera),
        };
        rcore.set_exit_key(KeyboardKey::Escape);
        rcore.init_window(self.window_size.0, self.window_size.1, &self.title);
        rcore.set_target_fps(60);

        for callback in self.setups.iter() {
            callback(rcore, rgestures, rcamera)?;
        }

        if let Some(scene) = self.scenes.last_mut() {
            scene.setup(connector)?
        }

        'mainloop: while !rcore.window_should_close() {
            let scene = match self.scenes.last_mut() {
                Some(scene) => scene,
                None => break,
            };

            match scene.update(connector, rcore.get_frame_time())? {
                State::Keep => (),
                State::Next(scene) => {
                    self.scenes.push(scene);
                    continue 'mainloop;
                }
                State::Prev => {
                    self.scenes.pop();
                    continue 'mainloop;
                }
                State::Quit => {
                    rcore.close_window();
                    break 'mainloop;
                }
            }

            rcore.begin_drawing();
            scene.draw_3d(Connector3D::default())?;
            scene.draw_2d(Connector2D::default())?;
            rcore.end_drawing();
        }

        Ok(())
    }
}

impl Default for Rscenes {
    /// Default window size is ¾ of the each screen dimension
    fn default() -> Self {
        let window_size = match current_resolution() {
            Ok((width, height)) => (width * 3 / 4, height * 3 / 4),
            _ => (800, 600),
        };
        Self {
            title: String::default(),
            window_size,
            setups: Vec::with_capacity(1),
            scenes: Vec::with_capacity(4),
        }
    }
}

pub trait SetupCallback = Fn(Rcore, Rgestures, Rcamera) -> Result<(), String> + 'static;
