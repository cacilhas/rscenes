use crate::connectors::*;
use crate::state::State;
use rscenes_raylib_connector::assets::*;
use std::fmt::Debug;

pub trait Scene: Debug + 'static {
    /// Implement get_camera_2d() to return your own camera
    fn get_camera_2d(&self) -> Camera2D {
        Camera2D::empty()
    }

    /// Implement get_camera_2d() to return your own camera
    fn get_camera_3d(&self) -> Camera3D {
        Camera3D::empty()
    }

    /// Implement setup() to run a procedure whenever the scene is reloaded
    #[allow(unused)]
    fn setup(&mut self, connector: PlainConnector) -> Result<(), String> {
        Ok(())
    }

    /// Implement exit() to run a procedure whenever exiting the schene
    #[allow(unused)]
    fn exit(&mut self, connector: PlainConnector) -> Result<(), String> {
        Ok(())
    }

    /// Implement #[draw(2d)] to render 2D objects
    #[allow(unused)]
    fn draw_2d(&self, connector: Connector2D) -> Result<(), String> {
        Ok(())
    }

    /// Implement #[draw(3d)] to render 3D objects
    #[allow(unused)]
    fn draw_3d(&self, connector: Connector3D) -> Result<(), String> {
        Ok(())
    }

    /// update() runs every loop
    #[allow(unused)]
    fn update(&mut self, connector: PlainConnector, dt: f32) -> Result<State, String> {
        Ok(State::Keep)
    }
}
