use std::cmp::max;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use Direction::*;
use ObjectType::*;

#[derive(Debug, Clone)]
enum ObjectType {
    ForwardMirror,      // /
    BackwardMirror,     // \
    HorizontalSplitter, // -
    VerticalSplitter,   // |
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        self.coords.x > max_x
            || self.coords.y > max_y
            || self.coords.x == 0 || self.coords.y == 0
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
            let max_x = split.next().map(|arr| arr.len()).unwrap_or(0);
            (max_x, max_y)
        };

        Self {
            max_x,
            max_y,
            objects,
        }
    }
}

fn path_length(mut area: Area, start_beam: Beam) -> usize {
    let mut energized = Energized::new(&area);

    let mut beams = vec![start_beam];

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

        beams.append(&mut new_beams);
        beams.retain(|beam| !beam.delete && !beam.is_oob(area.max_x, area.max_y));
    }

    energized.sum()
}

// This can be optimized but it already runs in 0.7s in Debug mode, so no need to
fn longest_edge_path(area: &Area) -> usize {
    let mut max_path = 0usize;

    fn pth_len(area: Area, x: usize, y: usize, direction: Direction) -> usize {
        path_length(
            area,
            Beam {
                coords: Coords { x, y },
                direction,
                delete: false,
            },
        )
    }

    // max_x == max_y in this case, but generalised solution is (almost) always better
    for idx in 1..area.max_x {
        max_path = max(max_path, pth_len(area.clone(), idx, 1, Down));
        max_path = max(max_path, pth_len(area.clone(), idx, area.max_y, Up));
    }

    for idx in 1..area.max_y {
        max_path = max(max_path, pth_len(area.clone(), 1, idx, Right));
        max_path = max(max_path, pth_len(area.clone(), area.max_x, idx, Left));
    }

    max_path
}

pub fn main() {
    let input = include_bytes!("../../input/day16");
    let area = Area::from_bytes(input);
    let p1 = path_length(
        area.clone(),
        Beam {
            direction: Right,
            coords: Coords { x: 1, y: 1 },
            delete: false,
        },
    );
    println!("part1 : {p1}");

    let p2 = longest_edge_path(&area);
    println!("part2 : {p2}");
}
