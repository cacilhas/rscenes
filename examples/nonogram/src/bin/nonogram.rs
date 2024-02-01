extern crate kodumaro_nonogram;

use kodumaro_nonogram::MainMenu;
use rscenes::prelude::*;

fn main() {
    // TraceLogLevel::Error.set_default();

    let mut manager = Rscenes::default();
    manager.title = "Nonogram".to_owned();
    manager.set_init(Box::new(MainMenu::default()));
    manager.add_setup(setup!(|rs| rs.set_exit_key(KeyboardKey::Null)));
    manager.add_setup(setup!(|rs| rs.init_audio_device()));
    manager.start();
}
