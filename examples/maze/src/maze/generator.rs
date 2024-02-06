use crate::direction;
use rand::seq::SliceRandom;
use rscenes::prelude::*;

#[derive(Debug)]
pub struct Generator<const W: usize, const H: usize> {
    pub width: usize,
    pub height: usize,
    rooms: Vec<Room>,
}

#[derive(Debug, Default)]
struct Room {
    directions: u8,
    visited: bool,
}

impl<const W: usize, const H: usize> Default for Generator<W, H> {
    fn default() -> Self {
        let width = W;
        let height = H;
        let rooms = (0..(width * height))
            .map(|_| Room::default())
            .collect::<Vec<Room>>();
        let mut generator = Self {
            width,
            height,
            rooms,
        };
        generator.init();
        generator
    }
}

impl<const W: usize, const H: usize> Generator<W, H> {
    fn init(&mut self) {
        let mut x = 0;
        let mut y = 0;
        let mut breadcrumb = Vec::with_capacity(self.width * self.height / 4);
        let mut rng = rand::thread_rng();

        loop {
            self.get(x, y).unwrap().visited = true;
            match self.neighbours(x, y).choose(&mut rng) {
                Some((lx, ly)) => {
                    let dir = direction![from(x, y) to(*lx, *ly)].unwrap();
                    {
                        let dir: u8 = dir.into();
                        self.get(x, y).unwrap().directions |= dir;
                    }
                    (x, y) = (*lx, *ly);
                    let dir: u8 = dir.compl().into();
                    self.get(x, y).unwrap().directions |= dir;
                    breadcrumb.push((x, y));
                }
                None => {
                    // No neighbours
                    match breadcrumb.pop() {
                        Some((lx, ly)) => (x, y) = (lx, ly),
                        None => return, // nowhere to get back
                    }
                }
            }
        }
    }

    fn get(&mut self, x: i32, y: i32) -> Option<&mut Room> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        }
        let index = x as usize + y as usize * self.width;
        Some(&mut self.rooms[index])
    }

    fn neighbours(&mut self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbours = Vec::<(i32, i32)>::with_capacity(4);
        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let (lx, ly) = (x + dx, y + dy);
            if let Some(room) = self.get(lx, ly) {
                if !room.visited {
                    neighbours.push((lx, ly));
                }
            }
        }
        neighbours
    }
}

pub struct GeneratorIter<'a, const W: usize, const H: usize> {
    generator: &'a mut Generator<W, H>,
    index: usize,
}

impl<'a, const W: usize, const H: usize> Iterator for GeneratorIter<'a, W, H> {
    type Item = (Vector3, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.generator.width * self.generator.height {
            let x = self.index % self.generator.width;
            let y = self.index / self.generator.width;
            let vector = Vector3::FORTH
                .mul(y as f32)
                .add(Vector3::RIGHT.mul(x as f32));
            let directions = self.generator.rooms[self.index].directions;
            self.index += 1;
            return Some((vector, directions));
        }
        None
    }
}

impl<'a, const W: usize, const H: usize> IntoIterator for &'a mut Generator<W, H> {
    type Item = (Vector3, u8);
    type IntoIter = GeneratorIter<'a, W, H>;

    fn into_iter(self) -> Self::IntoIter {
        GeneratorIter::<W, H> {
            generator: self,
            index: 0,
        }
    }
}
