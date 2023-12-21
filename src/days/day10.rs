use itertools::Itertools;
use std::ops::Index;
use Direction::*;
use Pipe::*;

struct Map(Vec<Vec<u8>>);

#[derive(Debug, PartialEq, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Pipe {
    Ground,
    NorthWest,
    NorthSouth,
    NorthEast,
    SouthWest,
    SouthEast,
    WestEast,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn enter_pipe(self, pipe: &Pipe) -> Direction {
        match (pipe, self) {
            (NorthSouth, North) => North,
            (NorthSouth, South) => South,
            (SouthEast, North) => East,
            (SouthEast, West) => South,
            (NorthWest, East) => North,
            (NorthWest, South) => West,
            (WestEast, West) => West,
            (WestEast, East) => East,
            (NorthEast, South) => East,
            (NorthEast, West) => North,
            (SouthWest, North) => West,
            (SouthWest, East) => South,
            _ => panic!(),
        }
    }
}

impl Pipe {
    fn is_enterable_from(&self, dir: Direction) -> bool {
        match self {
            NorthWest => dir == South || dir == East,
            NorthSouth => dir == South || dir == North,
            NorthEast => dir == South || dir == West,
            SouthWest => dir == North || dir == East,
            SouthEast => dir == North || dir == West,
            WestEast => dir == East || dir == West,
            Ground => false,
        }
    }
}

impl Coords {
    fn go_direction(&mut self, dir: Direction) {
        match dir {
            North => self.y = self.y.saturating_sub(1),
            South => self.y = self.y.saturating_add(1),
            East => self.x = self.x.saturating_add(1),
            West => self.x = self.x.saturating_sub(1),
        }
    }
}

impl TryFrom<u8> for Pipe {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Ground),
            b'|' => Ok(NorthSouth),
            b'F' => Ok(SouthEast),
            b'J' => Ok(NorthWest),
            b'-' => Ok(WestEast),
            b'L' => Ok(NorthEast),
            b'7' => Ok(SouthWest),
            _ => Err("Invalid!"),
        }
    }
}

impl Index<&Coords> for Map {
    type Output = u8;
    fn index(&self, idx: &Coords) -> &Self::Output {
        &self.0[idx.y][idx.x]
    }
}

impl Map {
    fn from_lines(lines: Vec<u8>) -> Self {
        Self(
            lines
                .split(|num| *num == b'\n')
                .map(Vec::from)
                .collect_vec(),
        )
    }

    fn find_start(&self) -> Coords {
        for (y, line) in self.0.iter().enumerate() {
            for (x, chr) in line.iter().enumerate() {
                if *chr == b'S' {
                    return Coords { x, y };
                }
            }
        }
        panic!("Couldn't find start coordinates!");
    }

    fn part1(&self) -> usize {
        let start_coords = self.find_start();
        let mut steps = 1;
        let mut curr_cords = start_coords.clone();

        let mut find_first_pipe = || -> Direction {
            if let Ok(pipe_north) =
                TryInto::<Pipe>::try_into(self.0[start_coords.y - 1][start_coords.x])
            {
                if pipe_north.is_enterable_from(North) {
                    curr_cords.go_direction(North);
                    return North.enter_pipe(&pipe_north);
                }
            }
            if let Ok(pipe_south) =
                TryInto::<Pipe>::try_into(self.0[start_coords.y + 1][start_coords.x])
            {
                if pipe_south.is_enterable_from(South) {
                    curr_cords.go_direction(South);
                    return South.enter_pipe(&pipe_south);
                }
            }
            if let Ok(pipe_east) =
                TryInto::<Pipe>::try_into(self.0[start_coords.y][start_coords.x + 1])
            {
                if pipe_east.is_enterable_from(East) {
                    curr_cords.go_direction(East);
                    return East.enter_pipe(&pipe_east);
                }
            }
            if let Ok(pipe_west) =
                TryInto::<Pipe>::try_into(self.0[start_coords.y][start_coords.x - 1])
            {
                if pipe_west.is_enterable_from(West) {
                    curr_cords.go_direction(West);
                    return West.enter_pipe(&pipe_west);
                }
            }
            panic!()
        };

        let mut curr_dir = find_first_pipe();

        loop {
            curr_cords.go_direction(curr_dir.clone());
            if let Ok(pipe) = TryInto::<Pipe>::try_into(self[&curr_cords]) {
                curr_dir = curr_dir.enter_pipe(&pipe);
                steps += 1;
            } else {
                break;
            }
        }

        (steps + 2 - 1) / 2
    }
}

pub fn main() {
    let map = Map::from_lines(include_bytes!("../../input/day10").to_vec());
    let p1 = map.part1();
    println!("part 1: {p1}");
}
