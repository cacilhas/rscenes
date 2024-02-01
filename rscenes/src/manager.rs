use crate::{connectors::*, scene::Scene, state::State};
use resolution::current_resolution;
use rscenes_raylib_connector::{
    assets::{TraceLogLevel, TraceLogLevelExt},
    interface::*,
};

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
    pub fn start(&mut self) {
        if self.scenes.is_empty() {
            TraceLogLevel::Fatal.log("no initial scene supplied");
        }
        if let Err(err) = self.setup() {
            TraceLogLevel::Fatal.log(format!("loading setup: {}", err));
        }

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
                if let Err(err) = scene.setup(plain_connector) {
                    TraceLogLevel::Fatal.log(format!("reloading {:?} scene: {}", scene, err));
                }
                reloaded = false;
            }

            plain_connector.begin_drawing();
            match scene.update(plain_connector, plain_connector.get_frame_time()) {
                Ok(State::Keep) => {
                    if let Err(err) = scene.draw_3d(connector_3d) {
                        TraceLogLevel::Error
                            .log(format!("drawing models (3D): {:?} {}", scene, err));
                    }
                    if let Err(err) = scene.draw_2d(connector_2d) {
                        TraceLogLevel::Error
                            .log(format!("drawing shapes (2D): {:?} {}", scene, err));
                    }
                }

                Ok(State::Next(next_scene)) => {
                    {
                        if let Err(err) = scene.exit(plain_connector) {
                            TraceLogLevel::Error.log(format!("exiting {:?} scene: {}", scene, err));
                        }
                    }
                    self.scenes.push(next_scene);
                    reloaded = true;
                }

                Ok(State::Prev) => {
                    if let Some(mut scene) = self.scenes.pop() {
                        if let Err(err) = scene.exit(plain_connector) {
                            TraceLogLevel::Error.log(format!("exiting {:?} scene: {}", scene, err));
                        }
                    }
                    reloaded = true;
                }

                Ok(State::Quit) => {
                    plain_connector.close_window();
                }

                Err(err) => {
                    TraceLogLevel::Error.log(format!("updating {:?} scene: {}", scene, err))
                }
            }
            plain_connector.end_drawing();
        }
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
