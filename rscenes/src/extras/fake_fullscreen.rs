use crate::{connectors::PlainConnector, setup};
use rscenes_raylib_connector::{assets::*, interface::*};

//const MASK: usize = ((ConfigFlags::InterlacedHint as usize) << 1) - 1;
const MASK: usize = ((ConfigFlags::WindowMousePassthrough as usize) << 1) - 1;

const POSITIVE_FLAGS: usize = ConfigFlags::WindowTopmost as usize
    // | ConfigFlags::BorderlessWindowedMode as usize
    | ConfigFlags::WindowUndecorated as usize;

const NEGATIVE_FLAGS: usize =
    ConfigFlags::WindowResizable as usize | ConfigFlags::FullscreenMode as usize;

const FAKE_FLAGS: usize = MASK & (POSITIVE_FLAGS ^ NEGATIVE_FLAGS);

static mut OTHER_FLAGS: Option<usize> = None;

pub fn start_fullscreen(other_flags: usize) -> Box<dyn Fn(PlainConnector) -> Result<(), String>> {
    if other_flags != 0 {
        unsafe {
            OTHER_FLAGS = Some(other_flags);
        }
    }
    Box::new(setup!(
        move |connector| connector.set_config_flags(FAKE_FLAGS | other_flags)
    ))
}

pub trait FakeFullscreen {
    /// Implement this method
    fn get_geometry_mut(&mut self) -> &mut Vector2;

    /// Call me in .on_update() method
    fn update_geometry(&mut self, connector: impl Rcore) {
        if connector.is_window_fullscreen() || connector.is_window_state(FAKE_FLAGS) {
            return;
        }

        let render = connector.get_render_rec();
        if render.width >= 800.0 && render.height >= 600.0 {
            let geom = self.get_geometry_mut();
            geom.x = render.width;
            geom.y = render.height;
        }
    }

    /// Call me to toggle fake fullscreen
    fn toggle_fake_fullscreen(&mut self, connector: impl Rcore + Copy) {
        if self.is_fullscreen_faked(connector) {
            let geom = self.get_geometry_mut();
            connector.clear_window_state(FAKE_FLAGS);
            if let Some(other_flags) = unsafe { OTHER_FLAGS } {
                connector.set_window_state(other_flags);
            }
            connector.set_window_size(geom.x as i32, geom.y as i32);
        } else {
            let monitor = connector.get_monitor_rec(connector.get_current_monitor());
            connector.set_window_state(FAKE_FLAGS);
            connector.set_window_size(monitor.width as i32, monitor.height as i32);
        }
    }

    /// Tell whether fullscreen is currently faked
    fn is_fullscreen_faked(&self, connector: impl Rcore) -> bool {
        connector.is_window_state(MASK & POSITIVE_FLAGS)
    }
}
