mod game;
mod ui;

use rscenes::prelude::*;
use ui::MainMenu;

fn main() {
    // TraceLogLevel::Error.set_default();

    let mut manager = Rscenes::default();
    manager.title = "Nonogram".to_owned();
    manager.set_init(Box::new(MainMenu::default()));
    manager.add_setup(setup!(|rs| rs.set_exit_key(KeyboardKey::Null)));
    manager.start();
}
