mod foe;
mod player;

use crate::player::Player;
use foe::Foe;
use rscenes::prelude::*;

fn main() -> Result<(), String> {
    TraceLogLevel::Error.set_default();

    let mut manager = Rscenes::default();
    manager.title = "Rscenes Test".to_owned();
    manager.set_init(Box::new(BallsScene {
        player: Player::default(),
        foes: (0..4).map(|_| Foe::default()).collect::<Vec<_>>(),
    }));
    setup![manager, |con| con.init_audio_device()];
    manager.start()
}

#[derive(Debug)]
struct BallsScene {
    player: Player,
    foes: Vec<Foe>,
}

impl Scene for BallsScene {
    fn setup(&mut self, connector: PlainConnector) -> Result<(), String> {
        let width = connector.get_render_width();
        let height = connector.get_render_height();
        self.player.x = (width - self.player.ball.width) as f32 / 2.0;
        self.player.y = (height - self.player.ball.height) as f32 / 2.0;
        for foe in self.foes.iter_mut() {
            foe.setup(connector)?;
        }
        Ok(())
    }

    fn update(&mut self, connector: PlainConnector, dt: f32) -> Result<State, String> {
        self.player.update(connector, dt)?;
        for foe in self.foes.iter_mut() {
            foe.update(connector, dt);
        }
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

        for foe in self.foes.iter() {
            foe.draw(connector)?;
        }
    }
}
