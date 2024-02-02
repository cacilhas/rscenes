use rscenes_raylib_connector::{assets::*, interface::*};

pub trait FakeFullscreen {
    /// Implement this method
    fn get_geometry_mut(&mut self) -> &mut Vector2;

    /// Call me in .on_update() method
    fn update_geometry(&mut self, connector: impl Rcore) {
        if connector.is_window_fullscreen()
            || connector.is_window_state(ConfigFlags::BorderlessWindowedMode.into())
        {
            return;
        }

        let render = connector.get_render_size();
        if render.x >= 800.0 && render.y >= 600.0 {
            let geom = self.get_geometry_mut();
            geom.x = render.x;
            geom.y = render.y;
        }
    }

    /// Call me to toggle fake fullscreen
    fn toggle_fake_fullscreen(&mut self, connector: impl Rcore + Copy) {
        if self.is_fullscreen_faked(connector) {
            let geom = self.get_geometry_mut();
            connector.set_window_size(geom.x as i32, geom.y as i32);
            connector.clear_window_state(ConfigFlags::WindowTopmost.into());
        } else {
            let screen = connector.get_screen_size();
            connector.set_window_size(screen.x as i32, screen.y as i32);
            connector.set_window_state(ConfigFlags::WindowTopmost.into());
        }
        connector.toggle_borderless_windowed();
    }

    /// Tell whether fullscreen is currently faked
    fn is_fullscreen_faked(&self, connector: impl Rcore) -> bool {
        connector.is_window_state(ConfigFlags::BorderlessWindowedMode.into())
    }
}
