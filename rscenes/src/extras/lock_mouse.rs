use rscenes_raylib_connector::{assets::*, interface::*};

use crate::connectors::PlainConnector;

/// Get mouse delta instead of position
pub trait LockMouse: Sized + Rcore {
    /// Keep mouse pointer on the render centre
    fn lock_mouse(self) -> Vector2 {
        let screen = self.get_render_rec();
        let center = Vector2 {
            x: screen.width / 2.0,
            y: screen.height / 2.0,
        };
        let mouse = self.get_mouse_position();
        self.set_mouse_position(center.x as i32, center.y as i32);
        center.add(mouse.mul(-1.0))
    }
}

impl LockMouse for PlainConnector {}
