use crate::{
    connectors::{Connector2D, Connector3D},
    scene::Scene,
    state::State,
};
use resolution::current_resolution;
use rscenes_raylib_connector::interface::*;

/// Control the game
#[derive(Default)]
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
        rcore.set_target_fps(60);
        let (width, height) = match self.window_size {
            (0, 0) => current_resolution().unwrap_or((800, 600)),
            (0, height) => {
                let (width, _) = current_resolution().unwrap_or((800, 600));
                (width, height)
            }
            (width, 0) => {
                let (_, height) = current_resolution().unwrap_or((800, 600));
                (width, height)
            }
            (width, height) => (width, height),
        };
        rcore.init_window(width, height, &self.title);

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

pub trait SetupCallback = Fn(Rcore, Rgestures, Rcamera) -> Result<(), String> + 'static;
