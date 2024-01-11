use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use Direction::*;
use ObjectType::*;

const TTL: usize = 700usize;
#[derive(Debug)]
enum ObjectType {
    ForwardMirror,      // /
    BackwardMirror,     // \
    HorizontalSplitter, // -
    VerticalSplitter,   // |
}

#[derive(Debug)]
struct Object {
    r#type: ObjectType,
    // left, right, up, down
    used: [bool; 4],
}

impl Object {
    fn new(r#type: ObjectType) -> Self {
        Self {
            r#type,
            used: [false; 4],
        }
    }

    fn r#use(&mut self, direction: &Direction) {
        match (direction, &self.r#type) {
            (Left, HorizontalSplitter)
            | (Right, HorizontalSplitter)
            | (Left, VerticalSplitter)
            | (Right, VerticalSplitter) => {
                self.used[0] = true;
                self.used[1] = true;
            }
            (Up, HorizontalSplitter)
            | (Down, HorizontalSplitter)
            | (Up, VerticalSplitter)
            | (Down, VerticalSplitter) => {
                self.used[2] = true;
                self.used[3] = true;
            }
            (Left, _) => self.used[0] = true,
            (Right, _) => self.used[1] = true,
            (Up, _) => self.used[2] = true,
            (Down, _) => self.used[3] = true,
        }
    }

    fn used_at(&self, direction: &Direction) -> bool {
        match direction {
            Left => self.used[0],
            Right => self.used[1],
            Up => self.used[2],
            Down => self.used[3],
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn go_direction(&mut self, direction: &Direction) {
        match direction {
            Left => self.x -= 1,
            Right => self.x += 1,
            Up => self.y -= 1,
            Down => self.y += 1,
        }
    }
}

#[derive(Debug)]
struct Area {
    objects: HashMap<Coords, Object>,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug)]
struct Beam {
    direction: Direction,
    coords: Coords,
    delete: bool,
}

struct Energized {
    row_len: usize,
    fields: Vec<bool>,
}

impl Energized {
    fn new(area: &Area) -> Self {
        Self {
            fields: vec![false; area.max_x * area.max_y],
            row_len: area.max_x,
        }
    }

    fn sum(&self) -> usize {
        self.fields.iter().filter(|b| **b).count()
    }
}

impl Index<Coords> for Energized {
    type Output = bool;
    fn index(&self, index: Coords) -> &Self::Output {
        &self.fields[(index.x - 1) + (index.y - 1) * self.row_len]
    }
}

impl IndexMut<Coords> for Energized {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        &mut self.fields[(index.x - 1) + (index.y - 1) * self.row_len]
    }
}

impl Beam {
    fn hit_object(&mut self, hit_object: &ObjectType) -> Option<Self> {
        match (&self.direction, hit_object) {
            (Left, ForwardMirror) | (Right, BackwardMirror) => {
                self.direction = Down;
                None
            }
            (Right, ForwardMirror) | (Left, BackwardMirror) => {
                self.direction = Up;
                None
            }
            (Up, ForwardMirror) | (Down, BackwardMirror) => {
                self.direction = Right;
                None
            }
            (Down, ForwardMirror) | (Up, BackwardMirror) => {
                self.direction = Left;
                None
            }
            (Left, HorizontalSplitter)
            | (Right, HorizontalSplitter)
            | (Up, VerticalSplitter)
            | (Down, VerticalSplitter) => None,
            (Up, HorizontalSplitter) | (Down, HorizontalSplitter) => {
                self.direction = Left;
                let second = Beam {
                    coords: self.coords,
                    direction: Right,
                    delete: false,
                };
                Some(second)
            }
            (Left, VerticalSplitter) | (Right, VerticalSplitter) => {
                self.direction = Up;
                let second = Beam {
                    coords: self.coords,
                    direction: Down,
                    delete: false,
                };
                Some(second)
            }
        }
    }

    fn tick(&mut self) {
        self.coords.go_direction(&self.direction);
    }

    fn is_oob(&self, max_x: usize, max_y: usize) -> bool {
        if self.coords.x > max_x
            || self.coords.y > max_y
            || self.coords.x == 0
            || self.coords.y == 0
        {
            true
        } else {
            false
        }
    }
}

impl Area {
    fn from_bytes(bytes: &[u8]) -> Self {
        let objects: HashMap<Coords, Object> = bytes
            .split(|byte| *byte == b'\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, byte)| {
                    let coords = Coords { x: x + 1, y: y + 1 };
                    match byte {
                        b'\\' => Some((coords, Object::new(BackwardMirror))),
                        b'/' => Some((coords, Object::new(ForwardMirror))),
                        b'|' => Some((coords, Object::new(VerticalSplitter))),
                        b'-' => Some((coords, Object::new(HorizontalSplitter))),
                        _ => None,
                    }
                })
            })
            .collect();

        let (max_x, max_y) = {
            let mut split = bytes.split(|byte| *byte == b'\n');
            let max_y = split.clone().count() - 1;
            let max_x = split.next().and_then(|arr| Some(arr.len())).unwrap_or(0);
            (max_x, max_y)
        };

        Self {
            max_x,
            max_y,
            objects,
        }
    }
}

fn solve_part1(mut area: Area) -> usize {
    let mut energized = Energized::new(&area);

    let mut beams = vec![Beam {
        coords: Coords { x: 1, y: 1 },
        direction: Right,
        delete: false,
    }];

    while !beams.is_empty() {
        let mut new_beams: Vec<Beam> = vec![];
        for beam in beams.iter_mut() {
            if let Some(obj) = area.objects.get_mut(&beam.coords) {
                if !obj.used_at(&beam.direction) {
                    obj.r#use(&beam.direction);
                    let new_beam = beam.hit_object(&obj.r#type);
                    if let Some(bm) = new_beam {
                        new_beams.push(bm);
                    }
                } else {
                    beam.delete = true;
                }
            }
            energized[beam.coords] = true;
            beam.tick();
        }

        for beam in new_beams.drain(..) {
            beams.push(beam);
        }
        beams.retain(|beam| !beam.is_oob(area.max_x, area.max_y) && !beam.delete);
    }

    energized.sum()
}

pub fn main() {
    let input = include_bytes!("../../input/day16");
    let area = Area::from_bytes(input);
    let p1 = solve_part1(area);
    println!("part : {p1}");
}
