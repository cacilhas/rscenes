use rscenes::prelude::*;

fn main() -> Result<(), String> {
    let mut manager = Rscenes::default();
    manager.title = "Rscenes Test".to_owned();
    manager.set_init(Box::new(BallsScene { ball: None }));
    manager.add_setup(load_assets);
    manager.start()
}

#[derive(Debug)]
struct BallsScene {
    pub ball: Option<Image>,
}

impl Scene for BallsScene {
    #[draw(shapes)]
    fn draw(&self) {
        let width = rcore.get_render_width();
        let height = rcore.get_render_height();
        rcore.clear_background(Color::CYAN);
        rshapes.draw_rectangle(0, 0, width / 2, height, Color::GREEN);
        if let Some(ball) = self.ball {
            rtextures.draw_texture(Texture2D::load_from_image(ball)?, 0, 0, Color::WHITE);
        }
    }
}

fn load_assets(scene: &mut Box<dyn Scene>, _connector: RaylibConnector) -> Result<(), String> {
    if let Some(scene) = Rscenes::scene_downcast_mut::<BallsScene>(scene) {
        let data = include_bytes!("./assets/ball_blue_large.png");
        scene.ball = Some(Image::load_from_memory(ImageType::Png, data)?);
    }
    Ok(())
}
