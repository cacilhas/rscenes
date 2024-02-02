use super::pause::Pause;
use crate::{
    audio::{Sfx, SfxManager, SfxType},
    game::{Board, Cell},
};
use rscenes::{
    extras::{FakeFullscreen, XDGStore},
    prelude::*,
};

const VICTORY: [&str; 6] = ["W", "w", "v", ".", "v", "w"];

#[derive(Debug)]
pub struct Gameplay {
    sfx: Option<Sfx>,
    board: Box<dyn Board>,
    hhints: Vec<String>,
    vhints: Vec<String>,
    size: Vector2,
    board_rect: Rectangle,
    hhints_rect: Rectangle,
    vhints_rect: Rectangle,
    cell_size: Vector2,
    time_lapse: f32,
    vic_index: f32,
    mute: bool,
    done: bool,
    left_click: bool,
    right_click: bool,
    geom: Vector2,
}

impl Gameplay {
    pub fn new(board: Box<dyn Board>, geom: Vector2) -> Self {
        let (w, h) = board.size();
        let size = Vector2 {
            x: w as f32,
            y: h as f32,
        };
        let hhints = (0..w)
            .map(|x| {
                board
                    .get_hhint(x)
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        let vhints = (0..h)
            .map(|y| {
                board
                    .get_vhint(y)
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        Self {
            sfx: None,
            board,
            size,
            hhints,
            vhints,
            board_rect: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            hhints_rect: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            vhints_rect: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            cell_size: Vector2::ZERO,
            time_lapse: 0.0,
            vic_index: 0.0,
            mute: false,
            done: false,
            left_click: false,
            right_click: false,
            geom,
        }
    }

    fn play(&self, sound: SfxType) {
        if !(self.mute) {
            if let Some(sfx) = &self.sfx {
                sfx.play(sound);
            }
        }
    }

    fn draw_lines(&self, rl: Connector2D, font: Font, mouse: Vector2) {
        let factor = match self.board.size() {
            (_, 5) => 0.5,
            _ => 0.625,
        };
        let font_size = if self.cell_size.x < self.cell_size.y {
            self.cell_size.x
        } else {
            self.cell_size.y
        } * factor
            - 2.0;

        self.draw_vertical_lines(
            rl,
            font,
            font_size,
            (((mouse.x - self.hhints_rect.x) / self.cell_size.x) + 0.5).floor() as usize,
        );
        self.draw_horizontal_lines(
            rl,
            font,
            font_size,
            ((mouse.y - self.vhints_rect.y) / self.cell_size.y).floor() as usize,
        );
    }

    fn draw_vertical_lines(&self, rl: Connector2D, font: Font, font_size: f32, mouse_col: usize) {
        for i in 0..(self.size.x as usize) {
            let x = self.hhints_rect.x + (i as f32 * self.cell_size.x);
            let bg = if i % 2 == 0 {
                Color::LIGHTGRAY
            } else {
                Color::WHEAT
            };
            let bg = if i == mouse_col { Color::LIGHTPINK } else { bg };
            rl.draw_rectangle(
                (x - self.cell_size.x / 2.0) as i32,
                0,
                self.cell_size.x as i32,
                self.hhints_rect.height as i32,
                bg,
            );
            let mut y = 0.0;
            for text in self.hhints[i].split(' ') {
                rl.draw_text_ex(font, text, Vector2 { x, y }, font_size, 1.0, Color::BLACK);
                y += font_size;
            }
            rl.draw_line_ex(
                Vector2 {
                    x: self.board_rect.x + (i as f32 * self.cell_size.x),
                    y: self.board_rect.y,
                },
                Vector2 {
                    x: self.board_rect.x + (i as f32 * self.cell_size.x),
                    y: self.board_rect.y + (self.size.y * self.cell_size.y),
                },
                if i % 5 == 0 { 4.0 } else { 2.0 },
                if i % 5 == 0 {
                    Color::BLACK
                } else {
                    Color::DARKGRAY
                },
            );
        }
        rl.draw_line_ex(
            Vector2 {
                x: self.board_rect.x + (self.size.y * self.cell_size.x),
                y: self.board_rect.y,
            },
            Vector2 {
                x: self.board_rect.x + (self.size.x * self.cell_size.x),
                y: self.board_rect.y + (self.size.y * self.cell_size.y),
            },
            4.0,
            Color::BLACK,
        );
    }

    fn draw_horizontal_lines(&self, rl: Connector2D, font: Font, font_size: f32, mouse_row: usize) {
        for i in 0..(self.size.y as usize) {
            let y = self.vhints_rect.y + (i as f32 * self.cell_size.y) + 4.0;
            let bg = if i % 2 == 0 {
                Color::LIGHTGRAY
            } else {
                Color::WHEAT
            };
            let bg = if i == mouse_row { Color::LIGHTPINK } else { bg };
            rl.draw_rectangle(
                self.board_rect.width as i32 + 2,
                y as i32 - 4,
                self.vhints_rect.width as i32,
                self.cell_size.y as i32,
                bg,
            );
            let text = &self.vhints[i];
            rl.draw_text_ex(
                font,
                text,
                Vector2 {
                    x: self.vhints_rect.x,
                    y,
                },
                font_size,
                1.0,
                Color::BLACK,
            );
            rl.draw_line_ex(
                Vector2 {
                    x: self.board_rect.x,
                    y: self.board_rect.y + (i as f32 * self.cell_size.y),
                },
                Vector2 {
                    x: self.board_rect.x + (self.size.y * self.cell_size.x),
                    y: self.board_rect.y + (i as f32 * self.cell_size.y),
                },
                if i % 5 == 0 { 4.0 } else { 2.0 },
                if i % 5 == 0 {
                    Color::BLACK
                } else {
                    Color::DARKGRAY
                },
            );
        }
        rl.draw_line_ex(
            Vector2 {
                x: self.board_rect.x,
                y: self.board_rect.y + (self.size.y * self.cell_size.y),
            },
            Vector2 {
                x: self.board_rect.x + (self.size.x * self.cell_size.x),
                y: self.board_rect.y + (self.size.y * self.cell_size.y),
            },
            4.0,
            Color::BLACK,
        );
    }

    fn draw_info(&self, rl: Connector2D, screen: Vector2, font: Font) {
        let size = rl.measure_text_ex(font, "F toggle fullscreen", 12.0, 1.0);
        let x = screen.x - size.x - 4.0;
        let mut y = 28.0;
        rl.draw_text_ex(
            font,
            "F2 mute/unmute",
            Vector2 { x, y },
            12.0,
            1.0,
            Color::GRAY,
        );
        y += 14.0;
        rl.draw_text_ex(font, "F3 pause", Vector2 { x, y }, 12.0, 1.0, Color::GRAY);
        y += 14.0;
        rl.draw_text_ex(font, "ESC abort", Vector2 { x, y }, 12.0, 1.0, Color::GRAY);
        y += 14.0;
        rl.draw_text_ex(
            font,
            "F toggle fullscreen",
            Vector2 { x, y },
            12.0,
            1.0,
            Color::GRAY,
        );
    }
}

impl FakeFullscreen for Gameplay {
    fn get_geometry_mut(&mut self) -> &mut Vector2 {
        &mut self.geom
    }
}

impl Scene for Gameplay {
    fn on_load(&mut self, _: PlainConnector) -> Result<(), String> {
        self.sfx = SfxManager::get_instance();
        if self.sfx.is_none() {
            TraceLogLevel::Error.log("couldn't load sound effects");
        }
        Ok(())
    }

    fn on_update(&mut self, rl: PlainConnector, dt: f32) -> Result<State, String> {
        self.update_geometry(rl);

        if KeyboardKey::F2.is_released() {
            self.mute = !self.mute;
        }

        if KeyboardKey::F3.is_released()
            || KeyboardKey::Pause.is_released() && !self.board.is_done()
        {
            return Ok(State::Next(Box::new(Pause::new(self.geom))));
        }

        if KeyboardKey::F.is_released() {
            self.toggle_fake_fullscreen(rl);
            // rl.toggle_fullscreen();
        }

        let screen = rl.get_render_size();
        self.board_rect = Rectangle {
            x: 0.0,
            y: screen.y * 0.25,
            width: screen.x * 0.75,
            height: screen.y * 0.75,
        };
        self.cell_size = Vector2 {
            x: self.board_rect.width / self.size.x,
            y: self.board_rect.height / self.size.y,
        };
        self.hhints_rect = Rectangle {
            x: self.cell_size.x * 0.5,
            y: 0.0,
            width: screen.x * 0.75,
            height: screen.y * 0.25,
        };
        self.vhints_rect = Rectangle {
            x: self.board_rect.x + self.board_rect.width + self.cell_size.x * 0.5,
            y: self.board_rect.y,
            width: screen.x * 0.25,
            height: screen.y * 0.25,
        };

        let left_click = rl.is_mouse_button_released(MouseButton::Left);
        let right_click = rl.is_mouse_button_released(MouseButton::Right);
        let ctrl = KeyboardKey::LeftControl.is_down() || KeyboardKey::RightControl.is_down();
        self.right_click = right_click || (left_click && ctrl);
        self.left_click = left_click && !ctrl;

        if self.board.is_done() && !self.done {
            self.done = true;
            self.play(SfxType::CLAPPING);

            for y in 0..(self.size.y as usize) {
                for x in 0..(self.size.x as usize) {
                    if self.board.get(x, y)? == Cell::Closed {
                        self.board.set(x, y, Cell::No)?;
                    }
                }
            }
        }

        if self.done {
            self.vic_index += dt * 5.0;
        } else if rl.is_window_focused() {
            self.time_lapse += dt;

            let mouse = rl.get_mouse_position();

            for y in 0..(self.size.y as usize) {
                for x in 0..(self.size.x as usize) {
                    let rec = Rectangle {
                        x: self.board_rect.x + (x as f32) * self.cell_size.x,
                        y: self.board_rect.y + (y as f32) * self.cell_size.y,
                        width: self.cell_size.x,
                        height: self.cell_size.y,
                    };

                    if !self.board.is_done() {
                        if self.left_click && rl.check_collision_point_rec(mouse, rec) {
                            match self.board.get(x, y)? {
                                Cell::Yes => {
                                    self.board.set(x, y, Cell::Closed)?;
                                    self.play(SfxType::UNSET);
                                }
                                Cell::Closed => {
                                    self.board.set(x, y, Cell::Yes)?;
                                    self.play(SfxType::SET);
                                }
                                Cell::No => self.play(SfxType::ERROR),
                            }
                        }
                        if self.right_click && rl.check_collision_point_rec(mouse, rec) {
                            match self.board.get(x, y)? {
                                Cell::No => {
                                    self.board.set(x, y, Cell::Closed)?;
                                    self.play(SfxType::UNSET);
                                }
                                Cell::Closed => {
                                    self.board.set(x, y, Cell::No)?;
                                    self.play(SfxType::LOCK);
                                }
                                Cell::Yes => self.play(SfxType::ERROR),
                            }
                        }
                    }
                }
            }
        }

        if KeyboardKey::Escape.is_released() {
            Ok(State::Prev(1))
        } else {
            Ok(State::Keep)
        }
    }

    #[draw(shapes)]
    fn draw(&self, rl: Connector2D) {
        rl.clear_background(Color::WHEAT);
        if !rl.is_window_focused() {
            return Ok(());
        }
        let font = rl.get_default_font();
        let screen = rl.get_render_size();
        let mouse = rl.get_mouse_position();

        for y in 0..(self.size.y as usize) {
            for x in 0..(self.size.x as usize) {
                let rec = Rectangle {
                    x: self.board_rect.x + (x as f32) * self.cell_size.x,
                    y: self.board_rect.y + (y as f32) * self.cell_size.y,
                    width: self.cell_size.x,
                    height: self.cell_size.y,
                };

                match self.board.get(x, y).unwrap() {
                    Cell::No => {
                        rl.draw_line(
                            rec.x as i32,
                            rec.y as i32,
                            (rec.x + rec.width) as i32,
                            (rec.y + rec.height) as i32,
                            Color::DARKGRAY,
                        );
                        rl.draw_line(
                            rec.x as i32,
                            (rec.y + rec.height) as i32,
                            (rec.x + rec.width) as i32,
                            rec.y as i32,
                            Color::DARKGRAY,
                        );
                    }
                    Cell::Yes => rl.draw_rectangle(
                        rec.x as i32,
                        rec.y as i32,
                        rec.width as i32,
                        rec.height as i32,
                        Color::DARKBLUE,
                    ),
                    Cell::Closed => rl.draw_rectangle(
                        rec.x as i32,
                        rec.y as i32,
                        rec.width as i32,
                        rec.height as i32,
                        Color::LIGHTPINK,
                    ),
                }
            }
        }

        self.draw_lines(rl, font, mouse);

        if self.board.is_done() {
            let size = rl.measure_text("V", 240) as f32;
            let rect = Rectangle {
                x: self.vhints_rect.x,
                y: screen.y,
                width: size,
                height: 240.0,
            };
            let text = VICTORY[self.vic_index as usize % VICTORY.len()];
            let size = rl.measure_text(text, 240) as f32;
            rl.draw_text_ex(
                font,
                text,
                Vector2 {
                    x: rect.x + (rect.width - size) / 2.0,
                    y: rect.y,
                },
                240.0,
                0.0,
                Color::GREEN,
            );
        }

        let size = rl.measure_text_ex(font, "F toggle fullscreen", 12.0, 1.0);
        let mut x = screen.x - size.x;
        let time = {
            let secs = self.time_lapse as i32;
            let min = secs / 60;
            let hours = min / 60;
            format!("{:02}:{:02}:{:02}", hours, min % 60, secs % 60)
        };
        monospace(
            rl,
            font,
            &time,
            Vector2 { x, y: 4.0 },
            12.0,
            Color::DARKGRAY,
        );
        let size = rl.measure_text_ex(font, "M", 12.0, 2.0);
        x -= size.x + 8.0;
        if self.mute {
            rl.draw_text_ex(font, "M", Vector2 { x, y: 4.0 }, 12.0, 0.0, Color::BROWN);
            rl.draw_text_ex(font, "\\", Vector2 { x, y: 4.0 }, 12.0, 0.0, Color::RED);
        }
        self.draw_info(rl, screen, font);
    }
}

fn monospace(
    rl: Connector2D,
    font: Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    tint: Color,
) {
    for (i, c) in text.as_bytes().iter().enumerate() {
        let x = position.x + (i as f32) * font_size;
        let y = position.y;
        rl.draw_text_ex(
            font,
            &char::from(*c).to_string(),
            Vector2 { x, y },
            font_size,
            0.0,
            tint,
        );
    }
}

impl Drop for Gameplay {
    fn drop(&mut self) {
        let geom = (self.geom.x as i32, self.geom.y as i32);
        if let Err(err) = XDGStore::save("nonogram", "window", geom) {
            TraceLogLevel::Error.log(format!("error saving geometry: {:?}", err));
        }
    }
}
