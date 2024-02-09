extern crate kodumaro_nonogram;

use kodumaro_nonogram::*;
use rscenes::{extras::XDGStore, prelude::*};

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
    manager
        .set_init(Box::new(scene))
        .add_setup(setup!(|rl| rl.set_exit_key(KeyboardKey::Null)))
        .add_setup(setup!(|rl| rl.set_target_fps(30)))
        .add_setup(setup!(|rl| rl.set_window_min_size(800, 600)))
        .add_setup(setup!(|rl| rl.init_audio_device()))
        .add_setup(setup!(move |rl| {
            if fs {
                rl.set_window_state(
                    ConfigFlags::FullscreenMode as usize | ConfigFlags::WindowResizable as usize,
                );
            } else {
                rl.set_window_state(ConfigFlags::WindowResizable as usize);
            }
        }))
        .add_setup(setup!(|_rs| SfxManager::load_assets()))
        .start();
}
