mod player;

use crate::player::Player;
use rscenes::prelude::*;

fn main() -> Result<(), String> {
    let mut manager = Rscenes::default();
    manager.title = "Rscenes Test".to_owned();
    manager.set_init(Box::new(BallsScene {
        player: Player::default(),
    }));
    manager.start()
}

#[derive(Debug)]
struct BallsScene {
    player: Player,
}

impl Scene for BallsScene {
    fn setup(&mut self, connector: PlainConnector) -> Result<(), String> {
        let width = connector.get_render_width();
        let height = connector.get_render_height();
        self.player.x = (width - self.player.ball.width) as f32 / 2.0;
        self.player.y = (height - self.player.ball.height) as f32 / 2.0;
        Ok(())
    }

    fn update(&mut self, connector: PlainConnector, dt: f32) -> Result<State, String> {
        self.player.update(connector, dt)?;
        Ok(State::Keep)
    }

    #[draw(shapes)]
    fn draw(&self, connector: Connector2D) {
        let width = connector.get_render_width();
        let height = connector.get_render_height();

        connector.clear_background(Color::CYAN);
        connector.draw_rectangle(0, 0, width / 2, height, Color::GREEN);
        let width = connector.get_render_width();
        if self.player.x < (width - self.player.ball.width) as f32 / 2.0 {
            connector.draw_fps(5, 5);
        }
        self.player.draw(connector)?;
    }
}
