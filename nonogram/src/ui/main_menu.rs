use rscenes::prelude::*;

use crate::{game::board::BoardStruct, Gameplay};

const LB_5X5: usize = 0;
const LB_10X10: usize = 1;
const LB_15X15: usize = 2;
const LB_EASY: usize = 3;
const BT_COUNT: usize = 4;
const BUTTONS: [(usize, &'static str); BT_COUNT] = [
    (LB_5X5, "5×5"),
    (LB_10X10, "10×10"),
    (LB_15X15, "15×15"),
    (LB_EASY, "Easy"),
];

// Colours
const BACKGROUND: Color = Color::WHEAT;
const TITLE: Color = Color::DARKCYAN;
const FOREGROUND: Color = Color::DARKGRAY;
const HOVER: Color = Color::BLACK;
const EZCOLOR: Color = Color::DARKSLATEBLUE;

#[derive(Debug)]
pub struct MainMenu {
    buttons: [Rectangle; 4],
    hover: [bool; 4],
    easy: bool,
}

impl Scene for MainMenu {
    fn on_setup(&mut self, rs: PlainConnector) -> Result<(), String> {
        let font = rs.get_default_font();
        let screen_width = rs.get_render_width() as f32;

        let size = rs.measure_text_ex(font, "Nonogram", 84.0, 2.0);
        let mut bottom = size.y + 64.0;

        for (idx, text) in BUTTONS.iter() {
            let size = rs.measure_text_ex(font, *text, 64.0, 1.0);
            self.buttons[*idx] = Rectangle {
                x: (screen_width - size.x) / 2.0,
                y: bottom,
                width: size.x,
                height: size.y,
            };
            bottom += size.y + 12.0;
        }

        Ok(())
    }

    fn on_update(&mut self, rs: PlainConnector, _: f32) -> Result<State, String> {
        let mouse = rs.get_mouse_position();
        for idx in 0..BT_COUNT {
            let rec = self.buttons[idx];
            self.hover[idx] = rs.check_collision_point_rec(mouse, rec);
        }

        if rs.is_mouse_button_released(MouseButton::Left) {
            if rs.check_collision_point_rec(mouse, self.buttons[LB_5X5]) {
                return Ok(State::Next(Box::new(Gameplay::new(Box::new(
                    BoardStruct::<5, 5>::random(self.easy),
                )))));
            }
            if rs.check_collision_point_rec(mouse, self.buttons[LB_10X10]) {
                return Ok(State::Next(Box::new(Gameplay::new(Box::new(
                    BoardStruct::<10, 10>::random(self.easy),
                )))));
            }
            if rs.check_collision_point_rec(mouse, self.buttons[LB_15X15]) {
                return Ok(State::Next(Box::new(Gameplay::new(Box::new(
                    BoardStruct::<15, 15>::random(self.easy),
                )))));
            }
            if rs.check_collision_point_rec(mouse, self.buttons[LB_EASY]) {
                self.easy = !self.easy;
            }
        }

        // TODO: change to the next scene

        if KeyboardKey::Escape.is_released() {
            Ok(State::Quit)
        } else {
            Ok(State::Keep)
        }
    }

    #[draw(shapes)]
    fn draw(&self, rs: Connector2D) {
        let screen_width = rs.get_render_width() as f32;
        let font = rs.get_default_font();

        rs.clear_background(BACKGROUND);

        let size = rs.measure_text_ex(font, "Nonogram", 84.0, 2.0);
        let position = Vector2 {
            x: (screen_width - size.x) / 2.0,
            y: 2.0,
        };
        rs.draw_text_ex(font, "Nonogram", position, 84.0, 2.0, TITLE);

        if self.easy {
            rs.draw_rectangle_rec(self.buttons[LB_EASY], EZCOLOR);
        }

        for (idx, text) in BUTTONS.iter() {
            let rec = self.buttons[*idx];
            rs.draw_text_ex(
                font,
                *text,
                Vector2 { x: rec.x, y: rec.y },
                64.0,
                1.0,
                if *idx == LB_EASY {
                    if self.easy {
                        BACKGROUND
                    } else {
                        EZCOLOR
                    }
                } else if self.hover[*idx] {
                    HOVER
                } else {
                    FOREGROUND
                },
            );
        }
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            buttons: (0..BT_COUNT)
                .map(|_| Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            hover: (0..BT_COUNT)
                .map(|_| false)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            easy: false,
        }
    }
}