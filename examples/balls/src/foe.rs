use rand::random;
use rscenes::prelude::*;
use std::f32::consts::TAU;

const FOE_SPEED: f32 = 500.0;

#[derive(Debug)]
pub struct Foe {
    pub ball: Image,
    pub x: f32,
    pub y: f32,
    pub movement: Vector2,
    pub pluck_sounds: Vec<Sound>,
}

impl Foe {
    pub fn draw(&self, connector: Connector2D) -> Result<(), String> {
        connector.draw_texture(
            Texture2D::load_from_image(self.ball)?,
            self.x as i32,
            self.y as i32,
            Color::WHITE,
        );
        Ok(())
    }

    pub fn update(&mut self, connector: PlainConnector, dt: f32) {
        self.x += self.movement.x * dt;
        self.y += self.movement.y * dt;

        let min_x = 0.0;
        let min_y = 0.0;
        let max_x = (connector.get_render_width() - self.ball.width) as f32;
        let max_y = (connector.get_render_height() - self.ball.height) as f32;

        let mut play = false;

        if self.x < min_x {
            self.movement.x = self.movement.x.abs();
            self.x = min_x;
            play = true;
        }
        if self.y < min_y {
            self.movement.y = self.movement.y.abs();
            self.y = min_y;
            play = true;
        }
        if self.x > max_x {
            self.movement.x = -self.movement.x.abs();
            self.x = max_x;
            play = true;
        }
        if self.y > max_y {
            self.movement.y = -self.movement.y.abs();
            self.y = max_y;
            play = true;
        }

        if play {
            let pluck = self
                .pluck_sounds
                .get(random::<usize>() % self.pluck_sounds.len())
                .unwrap();
            if pluck.is_ready() {
                pluck.play();
            }
        }
    }

    pub fn setup(&mut self, connector: PlainConnector) -> Result<(), String> {
        let width = (connector.get_render_width() - self.ball.width) as f32;
        let height = (connector.get_render_height() - self.ball.height) as f32;
        let min_x = self.ball.width as f32 / 2.0;
        let min_y = self.ball.height as f32 / 2.0;
        self.x = random::<f32>() * width + min_x;
        self.y = random::<f32>() * height + min_y;
        let angle: f32 = random::<f32>() * TAU;
        self.movement = Vector2 {
            x: angle.cos() * FOE_SPEED,
            y: angle.sin() * FOE_SPEED,
        };

        let data = include_bytes!("assets/pluck_001.ogg");
        let wave = Wave::load_from_memory(WaveType::Ogg, data)?;
        self.pluck_sounds.push(Sound::load_from_wave(wave));
        wave.unload();

        let data = include_bytes!("assets/pluck_002.ogg");
        let wave = Wave::load_from_memory(WaveType::Ogg, data)?;
        self.pluck_sounds.push(Sound::load_from_wave(wave));
        wave.unload();

        Ok(())
    }
}

impl Default for Foe {
    fn default() -> Self {
        Self {
            ball: Image::load("src/assets/ball_red_large.png").unwrap(),
            x: 0.0,
            y: 0.0,
            movement: Vector2 { x: 0.0, y: 0.0 },
            pluck_sounds: vec![],
        }
    }
}

impl Drop for Foe {
    fn drop(&mut self) {
        self.ball.unload();
        for sound in self.pluck_sounds.iter_mut() {
            sound.unload();
        }
    }
}