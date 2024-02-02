use rscenes::{extras::FakeFullscreen, prelude::*};

#[derive(Debug)]
pub struct Pause {
    geom: Vector2,
}

impl Pause {
    pub fn new(geom: Vector2) -> Self {
        Self { geom }
    }
}

impl FakeFullscreen for Pause {
    fn get_geometry_mut(&mut self) -> &mut Vector2 {
        &mut self.geom
    }
}

impl Scene for Pause {
    fn on_update(&mut self, rl: PlainConnector, _: f32) -> Result<State, String> {
        self.update_geometry(rl);

        if KeyboardKey::F.is_released() {
            self.toggle_fake_fullscreen(rl);
            // rl.toggle_fullscreen();
        }

        if KeyboardKey::F3.is_released() || KeyboardKey::Pause.is_released() {
            Ok(State::Prev(1))
        } else if KeyboardKey::Escape.is_released() {
            Ok(State::Prev(2))
        } else {
            Ok(State::Keep)
        }
    }

    #[draw(shapes)]
    fn draw(&self, rl: Connector2D) {
        let font = rl.get_default_font();
        let screen = rl.get_render_size();
        rl.clear_background(Color::WHEAT);

        let text = "Nonogram";
        let size = rl.measure_text_ex(font, text, 84.0, 2.0);
        let position = Vector2 {
            x: (screen.x - size.x) / 2.0,
            y: 0.0,
        };
        rl.draw_text_ex(font, text, position, 84.0, 2.0, Color::DARKCYAN);
        let mut bottom = size.y + 64.0;

        let text = "PAUSED";
        let size = rl.measure_text_ex(font, text, 84.0, 2.0);
        let position = Vector2 {
            x: (screen.x - size.x) / 2.0,
            y: bottom,
        };
        rl.draw_text_ex(font, text, position, 84.0, 2.0, Color::BROWN);
        bottom += size.y + 64.0;

        let text = "F3 or Pause resume game";
        let size = rl.measure_text_ex(font, text, 32.0, 2.0);
        let position = Vector2 {
            x: (screen.x - size.x) / 2.0,
            y: bottom,
        };
        rl.draw_text_ex(font, text, position, 32.0, 2.0, Color::BLACK);
        bottom += size.y + 32.0;

        let text = "Escape to abort";
        let size = rl.measure_text_ex(font, text, 32.0, 2.0);
        let position = Vector2 {
            x: (screen.x - size.x) / 2.0,
            y: bottom,
        };
        rl.draw_text_ex(font, text, position, 32.0, 2.0, Color::BLACK);
    }
}
