extern crate kodumaro_nonogram;

use kodumaro_nonogram::*;
use rscenes::{
    extras::{start_fullscreen, XDGStore},
    prelude::*,
};

fn main() {
    // TraceLogLevel::Error.set_default();
    XDGStore::init_storage("nonogram").unwrap();

    let mut manager = Rscenes::default();
    manager.title = "Nonogram".to_owned();

    let (width, height, fs) = match XDGStore::retrieve::<Persist>("nonogram", "window") {
        Ok(value) => value,

        _ => {
            let (width, height) = manager.screen_size();
            (width * 2 / 3, height * 2 / 3, false)
        }
    };
    let (width, height) = (width.max(800), height.max(600));
    manager.window_size = (width, height);

    let mut scene = MainMenu::default();
    scene.geom.x = width as f32;
    scene.geom.y = height as f32;
    manager.set_init(Box::new(scene));
    manager.add_setup(setup!(|rs| rs.set_exit_key(KeyboardKey::Null)));
    if fs {
        manager.add_setup(start_fullscreen(ConfigFlags::WindowResizable.into()));
    } else {
        manager.add_setup(setup!(
            |rs| rs.set_window_state(ConfigFlags::WindowResizable.into())
        ));
    }
    manager.add_setup(setup!(|rs| rs.set_window_min_size(800, 600)));
    manager.add_setup(setup!(|rs| rs.init_audio_device()));
    manager.add_setup(setup!(|_rs| SfxManager::load_assets()));
    manager.start();
}
