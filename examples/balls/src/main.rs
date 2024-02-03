mod foe;
mod player;

use crate::player::Player;
use foe::Foe;
use rscenes::prelude::*;

fn main() {
    TraceLogLevel::Error.set_default();

    let mut manager = Rscenes::default();
    manager.title = "Rscenes Test".to_owned();
    manager
        .set_init(Box::new(BallsScene {
            player: Player::default(),
            foes: (0..4).map(|_| Foe::default()).collect::<Vec<_>>(),
            game_over: false,
            collision_sound: None,
        }))
        .add_setup(setup!(|con| con.init_audio_device()))
        .start();
}

#[derive(Debug)]
struct BallsScene {
    player: Player,
    foes: Vec<Foe>,
    game_over: bool,
    collision_sound: Option<Sound>,
}

impl Scene for BallsScene {
    fn on_setup(&mut self, _: PlainConnector) -> Result<(), String> {
        let data = include_bytes!("assets/impactBell_heavy_000.ogg");
        let wave = Wave::load_from_memory(WaveType::Ogg, data)?;
        self.collision_sound = Some(Sound::load_from_wave(wave));
        Ok(())
    }

    fn on_load(&mut self, rl: PlainConnector) -> Result<(), String> {
        let screen = rl.get_render_rec();
        self.player.x = (screen.width - self.player.ball.width as f32) / 2.0;
        self.player.y = (screen.height - self.player.ball.height as f32) / 2.0;
        for foe in self.foes.iter_mut() {
            foe.setup(rl)?;
        }
        Ok(())
    }

    fn on_update(&mut self, rl: PlainConnector, dt: f32) -> Result<State, String> {
        if !self.game_over {
            self.player.update(rl, dt)?;
        }
        for foe in self.foes.iter_mut() {
            foe.update(rl, dt);
            if rl.check_collision_circles(
                (&self.player).into(),
                self.player.radius,
                (foe as &Foe).into(),
                foe.radius,
            ) {
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
    fn draw(&self, rl: Connector2D) {
        let screen = rl.get_render_rec();
        rl.clear_background(Color::CYAN);
        let green_rec = Rectangle {
            x: 0.0,
            y: 0.0,
            width: screen.width / 2.0,
            height: screen.height,
        };
        rl.draw_rectangle_rec(green_rec, Color::GREEN);
        if self.game_over
            || rl.check_collision_circle_rec((&self.player).into(), self.player.radius, green_rec)
        {
            rl.draw_fps(5, 5);
        }
        if !self.game_over {
            self.player.draw(rl)?;
        }

        for foe in self.foes.iter() {
            foe.draw(rl)?;
        }
    }
}
