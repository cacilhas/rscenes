use crate::connectors::*;
use crate::state::State;
use rscenes_raylib_connector::assets::*;
use std::fmt::Debug;
use std::ptr;

pub trait Scene: Debug + 'static {
    fn id(&self) -> usize {
        ptr::addr_of!(*self) as *const i32 as usize
    }

    /// Implement get_camera_2d() to return your own camera
    fn get_camera_2d(&self) -> Camera2D {
        Camera2D::empty()
    }

    /// Implement get_camera_2d() to return your own camera
    fn get_camera_3d(&self) -> Camera3D {
        Camera3D::empty()
    }

    /// Implement on_setup() to run a procedure first time the scene is loaded
    #[allow(unused)]
    fn on_setup(&mut self, connector: PlainConnector) -> Result<(), String> {
        Ok(())
    }

    /// Implement on_load() to run a procedure whenever the scene is reloaded
    #[allow(unused)]
    fn on_load(&mut self, connector: PlainConnector) -> Result<(), String> {
        Ok(())
    }

    /// Implement on_exit() to run a procedure whenever exiting the schene
    #[allow(unused)]
    fn on_exit(&mut self, connector: PlainConnector) -> Result<(), String> {
        Ok(())
    }

    /// Implement #[draw(shades)] to render 2D objects
    #[allow(unused)]
    fn draw_2d(&self, connector: Connector2D) -> Result<(), String> {
        Ok(())
    }

    /// Implement #[draw(models)] to render 3D objects
    #[allow(unused)]
    fn draw_3d(&self, connector: Connector3D) -> Result<(), String> {
        Ok(())
    }

    /// Implement #[draw(hud)] to render 2D objects on a head-up display
    #[allow(unused)]
    fn draw_hud(&self, connector: Connector2D) -> Result<(), String> {
        Ok(())
    }

    /// on_update() runs every loop
    #[allow(unused)]
    fn on_update(&mut self, connector: PlainConnector, dt: f32) -> Result<State, String> {
        Ok(State::Keep)
    }
}
