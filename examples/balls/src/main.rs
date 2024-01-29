use rscenes::prelude::*;

fn main() -> Result<(), String> {
    let mut manager = Rscenes::default();
    manager.title = "Rscenes Test".to_owned();
    manager.set_init(Box::new(BallsScene));
    manager.start()
}

#[derive(Debug)]
struct BallsScene;

impl Scene for BallsScene {
    #[draw(shapes)]
    fn draw(&self) {
        rcore.clear_background(Color::CYAN);
    }
}
