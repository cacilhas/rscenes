use crate::{connectors::*, scene::Scene, state::State};
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
        self.setup()?;

        let mut reloaded = true;
        let plain_connector = PlainConnector::default();
        let connector_3d = Connector3D::default();
        let connector_2d = Connector2D::default();

        'mainloop: while !plain_connector.window_should_close() {
            let scene = match self.scenes.last_mut() {
                Some(scene) => scene,
                None => break 'mainloop,
            };

            if reloaded {
                scene.setup(plain_connector)?;
                reloaded = false;
            }

            match scene.update(plain_connector, plain_connector.get_frame_time())? {
                State::Keep => {
                    plain_connector.begin_drawing();
                    scene.draw_3d(connector_3d)?;
                    scene.draw_2d(connector_2d)?;
                    plain_connector.end_drawing();
                }

                State::Next(next_scene) => {
                    {
                        scene.exit(plain_connector)?;
                    }
                    self.scenes.push(next_scene);
                    reloaded = true;
                }

                State::Prev => {
                    if let Some(mut scene) = self.scenes.pop() {
                        scene.exit(plain_connector)?;
                    }
                    reloaded = true;
                }

                State::Quit => {
                    plain_connector.close_window();
                }
            }
        }

        Ok(())
    }

    fn setup(&mut self) -> Result<(), String> {
        let connector = PlainConnector::default();

        connector.set_target_fps(60);
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
        connector.init_window(width, height, &self.title);
        for callback in self.setups.iter() {
            callback(connector)?;
        }

        Ok(())
    }
}

pub trait SetupCallback = Fn(PlainConnector) -> Result<(), String> + 'static;
