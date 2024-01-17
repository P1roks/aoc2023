use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

const STEP_COUNT: u8 = 64u8;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coords {
    x: usize,
    y: usize,
}

struct Map {
    occupied: Vec<bool>,
    row_len: usize,
}

impl Map {
    fn from_bytes(bytes: &[u8]) -> Self {
        let row_len = bytes.split(|byte| *byte == b'\n').count() - 1;
        let occupied = vec![false; row_len * row_len];
        let mut map = Self { occupied, row_len };

        for (y, line) in bytes.split(|byte| *byte == b'\n').enumerate() {
            for (x, chr) in line.iter().enumerate() {
                if *chr == b'#' {
                    map[(x, y)] = true;
                }
            }
        }

        map
    }

    fn get_possible_steps(&self, initial: Coords) -> [Option<Coords>; 4] {
        let mut coords: [Option<Coords>; 4] = [None; 4];

        if initial.x != 0 && !self[(initial.x - 1, initial.y)] {
            coords[0] = Some(Coords {
                x: initial.x - 1,
                y: initial.y,
            });
        }

        if initial.y != 0 && !self[(initial.x, initial.y - 1)] {
            coords[1] = Some(Coords {
                x: initial.x,
                y: initial.y - 1,
            });
        }

        if initial.x != (self.row_len - 1) && !self[(initial.x + 1, initial.y)] {
            coords[2] = Some(Coords {
                x: initial.x + 1,
                y: initial.y,
            });
        }

        if initial.y != (self.row_len - 1) && !self[(initial.x, initial.y + 1)] {
            coords[3] = Some(Coords {
                x: initial.x,
                y: initial.y + 1,
            });
        }
        coords
    }

    #[deprecated]
    fn print(&self) {
        let mut cnt = 0;
        for item in self.occupied.iter() {
            if cnt != 0 && cnt % self.row_len == 0 {
                println!();
            }

            let symbol = if *item { "#" } else { "." };
            print!("{symbol}");
            cnt += 1;
        }
        println!();
    }
}

impl Index<Coords> for Map {
    type Output = bool;
    fn index(&self, index: Coords) -> &Self::Output {
        &self.occupied[index.x + index.y * self.row_len]
    }
}

impl Index<(usize, usize)> for Map {
    type Output = bool;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.occupied[x + y * self.row_len]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.occupied[x + y * self.row_len]
    }
}

fn find_start(bytes: &[u8]) -> Coords {
    for (y, line) in bytes.split(|byte| *byte == b'\n').enumerate() {
        for (x, chr) in line.iter().enumerate() {
            if *chr == b'S' {
                return Coords { x, y };
            }
        }
    }
    panic!()
}

fn solve_part1(map: &Map, start: Coords) -> usize {
    let mut possible = HashSet::new();
    possible.insert(start);

    for _ in 0..STEP_COUNT {
        let mut old_steps = HashSet::<Coords>::new();
        std::mem::swap(&mut possible, &mut old_steps);
        for coord in old_steps.drain() {
            for new_pos in map.get_possible_steps(coord) {
                if let Some(new_pos) = new_pos {
                    possible.insert(new_pos);
                }
            }
        }
    }

    possible.iter().count()
}

pub fn main() {
    let input = include_bytes!("../../input/day21");
    let map = Map::from_bytes(input);
    let start = find_start(input);

    let p1 = solve_part1(&map, start);
    println!("part1: {p1}");
}
