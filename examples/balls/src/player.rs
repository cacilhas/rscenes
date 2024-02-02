use rscenes::prelude::*;

const PLAYER_SPEED: f32 = 500.0;

#[derive(Debug)]
pub struct Player {
    pub ball: Image,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Player {
    pub fn draw(&self, connector: Connector2D) -> Result<(), String> {
        connector.draw_texture(
            Texture2D::load_from_image(self.ball)?,
            self.x as i32,
            self.y as i32,
            Color::WHITE,
        );
        Ok(())
    }

    pub fn update(&mut self, connector: PlainConnector, dt: f32) -> Result<(), String> {
        self.r#move(dt);
        self.constraint(connector);
        Ok(())
    }

    fn r#move(&mut self, dt: f32) {
        if KeyboardKey::Left.is_down() || KeyboardKey::A.is_down() {
            self.x -= PLAYER_SPEED * dt;
        }
        if KeyboardKey::Right.is_down() || KeyboardKey::D.is_down() {
            self.x += PLAYER_SPEED * dt;
        }
        if KeyboardKey::Up.is_down() || KeyboardKey::W.is_down() {
            self.y -= PLAYER_SPEED * dt;
        }
        if KeyboardKey::Down.is_down() || KeyboardKey::S.is_down() {
            self.y += PLAYER_SPEED * dt;
        }
    }

    fn constraint(&mut self, connector: PlainConnector) {
        let width = connector.get_render_width();
        let height = connector.get_render_height();
        let min_x = 0.0;
        let min_y = 0.0;
        let max_x = (width - self.ball.width) as f32;
        let max_y = (height - self.ball.height) as f32;

        if self.x < min_x {
            self.x = min_x;
        }
        if self.x > max_x {
            self.x = max_x;
        }
        if self.y < min_y {
            self.y = min_y;
        }
        if self.y > max_y {
            self.y = max_y;
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        let data = include_bytes!("assets/ball_blue_large.png");
        let ball = Image::load_from_memory(ImageType::Png, data).unwrap();
        let radius = (ball.width + ball.height) as f32 / 4.0;

        Self {
            ball,
            x: 0.0,
            y: 0.0,
            radius,
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.ball.unload();
    }
}

impl From<&Player> for Vector2 {
    fn from(player: &Player) -> Self {
        Self {
            x: player.x,
            y: player.y,
        }
    }
}
