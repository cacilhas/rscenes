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
        game_over: false,
        collision_sound: None,
    }));
    manager.add_setup(setup!(|con| con.init_audio_device()));
    manager.start()
}

#[derive(Debug)]
struct BallsScene {
    player: Player,
    foes: Vec<Foe>,
    game_over: bool,
    collision_sound: Option<Sound>,
}

impl Scene for BallsScene {
    fn setup(&mut self, connector: PlainConnector) -> Result<(), String> {
        if self.collision_sound.is_none() {
            let data = include_bytes!("assets/impactBell_heavy_000.ogg");
            let wave = Wave::load_from_memory(WaveType::Ogg, data)?;
            self.collision_sound = Some(Sound::load_from_wave(wave));
        }

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
        if !self.game_over {
            self.player.update(connector, dt)?;
        }
        for foe in self.foes.iter_mut() {
            foe.update(connector, dt);
            let dx = (foe.x - self.player.x).powf(2.0);
            let dy = (foe.y - self.player.y).powf(2.0);
            let min = (foe.radius + self.player.radius).powf(2.0);
            if dx + dy < min {
                self.game_over = true;
                self.player.x = -2.0 * self.player.radius;
                if let Some(collision) = self.collision_sound {
                    if !collision.is_playing() {
                        collision.play();
                    }
                }
            }
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
        if self.game_over || self.player.x < (width - self.player.ball.width) as f32 / 2.0 {
            connector.draw_fps(5, 5);
        }
        if !self.game_over {
            self.player.draw(connector)?;
        }

        for foe in self.foes.iter() {
            foe.draw(connector)?;
        }
    }
}
