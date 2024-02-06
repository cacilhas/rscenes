mod maze;

use crate::maze::{Direction, Generator};
use rscenes::{
    extras::{start_fullscreen, LockMouse},
    prelude::*,
};
use std::f32::consts::TAU;

const SIZE: usize = 50;
const ROTATION_SPEED: f32 = TAU;
const SPEED: f32 = 2.0;

fn main() {
    let mut manager = Rscenes::default();
    manager.title = "Maze".to_owned();

    manager
        .set_init(Box::new(MazeScene::default()))
        .add_setup(start_fullscreen(0))
        .add_setup(setup!(|rl| rl.disable_cursor()))
        .start();
}

#[derive(Debug)]
struct MazeScene {
    camera: Camera3D,
    rooms: Vec<(Vector3, u8)>,
    width: usize,
    height: usize,
}

impl Default for MazeScene {
    fn default() -> Self {
        let mut generator = Generator::<SIZE, SIZE>::default();
        let rooms = generator.into_iter().collect::<Vec<_>>();
        let mut camera = Camera3D::empty();
        camera.move_by(camera.position.mul(-1.0).add(Vector3::UP.mul(1.6)));

        Self {
            camera,
            rooms,
            width: SIZE,
            height: SIZE,
        }
    }
}

impl Scene for MazeScene {
    fn get_camera_3d(&self) -> Camera3D {
        self.camera
    }

    fn on_update(&mut self, rl: PlainConnector, dt: f32) -> Result<State, String> {
        let mut delta = rl.lock_mouse();
        if delta.sqr_length() > 0.0 {
            if delta.x.abs() > ROTATION_SPEED {
                delta.x = ROTATION_SPEED * delta.x.signum();
            }
            if delta.y.abs() > ROTATION_SPEED {
                delta.y = ROTATION_SPEED * delta.y.signum();
            }
            let delta = delta.mul(dt);

            self.camera.rotate(delta.x, Vector3::UP);
            self.camera.rotate_local(delta.y, Vector3::RIGHT);
            let elevation = self.camera.front_vector().y_elevation_angle();
            let top = TAU / 6.0;
            let bottom = -TAU / 18.0;
            if elevation > top {
                self.camera.set_y_elevation(top);
            } else if elevation < bottom {
                self.camera.set_y_elevation(bottom);
            }
        }

        Ok(State::Keep)
    }

    #[draw(shapes)]
    fn draw(&self, rl: Connector2D) {
        // Skyblue background
        rl.clear_background(Color::SKYBLUE);
    }

    #[draw(models)]
    fn draw(&self, rl: Connector3D) {
        rl.draw_plane(
            Vector3::ZERO,
            Vector2::ONE.mul(SIZE as f32),
            Color::DARKGREEN,
        );

        for &(location, ways) in self.rooms.iter() {
            // TODO
        }
    }
}
