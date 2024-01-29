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
        let connector_3d = Connector3D::default();
        let connector_2d = Connector2D::default();
        let rcore = connector.rcore;

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

        if let Some(scene) = self.scenes.last_mut() {
            for callback in self.setups.iter() {
                callback(scene, connector)?;
            }
        }
        let mut reload = true;

        'mainloop: while !rcore.window_should_close() {
            let scene = match self.scenes.last_mut() {
                Some(scene) => scene,
                None => break 'mainloop,
            };

            if reload {
                scene.setup(connector)?;
                reload = false;
            }

            match scene.update(connector, rcore.get_frame_time())? {
                State::Keep => {
                    rcore.begin_drawing();
                    scene.draw_3d(connector_3d)?;
                    scene.draw_2d(connector_2d)?;
                    rcore.end_drawing();
                }

                State::Next(next_scene) => {
                    {
                        scene.exit(connector)?;
                    }
                    self.scenes.push(next_scene);
                    reload = true;
                }

                State::Prev => {
                    if let Some(mut scene) = self.scenes.pop() {
                        scene.exit(connector)?;
                    }
                    reload = true;
                }

                State::Quit => {
                    rcore.close_window();
                    break 'mainloop;
                }
            }
        }

        Ok(())
    }
}

pub trait SetupCallback = Fn(&mut Box<dyn Scene>, RaylibConnector) -> Result<(), String> + 'static;
