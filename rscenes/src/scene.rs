use std::fmt::Debug;

use crate::connectors::*;
use crate::state::State;
use rscenes_raylib_connector::{assets::*, interface::*};

pub trait Scene: Debug {
    fn get_camera_2d(&self) -> Camera2D {
        Camera2D::empty()
    }
    fn get_camera_3d(&self) -> Camera3D {
        Camera3D::empty()
    }

    #[allow(unused)]
    fn draw_2d(&self, connector: Connector2D) -> Result<(), String> {
        Ok(())
    }

    #[allow(unused)]
    fn draw_3d(&self, connector: Connector3D) -> Result<(), String> {
        Ok(())
    }

    #[allow(unused)]
    fn update(&mut self, connector: RaylibConnector) -> Result<State, String> {
        Ok(State::Keep)
    }
}