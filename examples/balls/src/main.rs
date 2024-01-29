use rscenes::prelude::*;

fn main() -> Result<(), String> {
    let mut manager = Rscenes::default();
    manager.title = "Rscenes Test".to_owned();
    manager.set_init(Box::new(BallsScene {
        ball: Image::load("src/assets/ball_blue_large.png")?,
        x: 0,
        y: 0,
    }));
    manager.start()
}

#[derive(Debug)]
struct BallsScene {
    ball: Image,
    x: i32,
    y: i32,
}

impl Scene for BallsScene {
    fn setup(&mut self, connector: PlainConnector) -> Result<(), String> {
        let width = connector.get_render_width();
        let height = connector.get_render_height();
        self.x = (width - self.ball.width) / 2;
        self.y = (height - self.ball.height) / 2;
        Ok(())
    }

    #[draw(shapes)]
    fn draw(&self, connector: Connector2D) {
        let width = connector.get_render_width();
        let height = connector.get_render_height();
        connector.clear_background(Color::CYAN);
        connector.draw_rectangle(0, 0, width / 2, height, Color::GREEN);
        connector.draw_texture(
            Texture2D::load_from_image(self.ball)?,
            self.x,
            self.y,
            Color::WHITE,
        );
    }
}
