extern crate kodumaro_nonogram;

use kodumaro_nonogram::*;
use rscenes::prelude::*;

fn main() {
    // TraceLogLevel::Error.set_default();

    let mut manager = Rscenes::default();
    manager.title = "Nonogram".to_owned();

    let (width, height) = manager.screen_size();
    let width = 800.max(width * 2 / 3);
    let height = 600.max(height * 2 / 3);
    manager.window_size = (width, height);

    manager.set_init(Box::new(MainMenu::default()));
    manager.add_setup(setup!(|rs| rs.set_exit_key(KeyboardKey::Null)));
    manager.add_setup(setup!(
        |rs| rs.set_window_state(ConfigFlags::WindowResizable.into())
    ));
    manager.add_setup(setup!(|rs| rs.set_window_min_size(800, 600)));
    manager.add_setup(setup!(|rs| rs.init_audio_device()));
    manager.add_setup(setup!(|_rs| SfxManager::load_assets()));
    manager.start();
}
