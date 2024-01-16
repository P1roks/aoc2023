use std::cmp::Reverse;

use indexmap::IndexMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Rock {
    Round,
    Cube,
}

#[derive(Debug, Clone)]
struct Rocks {
    rocks: Vec<(Coords, Rock)>,
    bounds: usize,
}

impl Rocks {
    fn from_bytes(bytes: &&[u8]) -> Self {
        let mut rocks = Vec::new();
        for (y, line) in bytes.split(|p| *p == b'\n').enumerate() {
            for (x, chr) in line.iter().enumerate() {
                match chr {
                    b'O' => rocks.push((Coords { x, y }, Rock::Round)),
                    b'#' => rocks.push((Coords { x, y }, Rock::Cube)),
                    _ => {}
                };
            }
        }
        let bounds = bytes.split(|byte| *byte == b'\n').count() - 1;

        Self { rocks, bounds }
    }

    fn shift_north(&mut self) {
        let Self { rocks, .. } = self;
        rocks.sort_by_key(|rock| (rock.0.x, rock.0.y));

        for (_, group) in &rocks.iter_mut().group_by(|rock| rock.0.x) {
            let mut low_y = 0;
            for (coords, rock_type) in group {
                match rock_type {
                    Rock::Cube => {
                        low_y = coords.y + 1;
                    }
                    Rock::Round => {
                        coords.y = low_y;
                        low_y += 1;
                    }
                };
            }
        }
    }

    fn shift_south(&mut self) {
        let Self { rocks, ref bounds } = self;
        rocks.sort_by_key(|rock| (rock.0.x, Reverse(rock.0.y)));

        for (_, group) in &rocks.iter_mut().group_by(|rock| rock.0.x) {
            let mut high_y = bounds - 1;
            for (coords, rock_type) in group {
                match rock_type {
                    Rock::Cube => {
                        high_y = coords.y.saturating_sub(1);
                    }
                    Rock::Round => {
                        coords.y = high_y;
                        high_y = high_y.saturating_sub(1);
                    }
                };
            }
        }
    }

    fn shift_west(&mut self) {
        let Self { rocks, .. } = self;
        rocks.sort_by_key(|rock| (rock.0.y, rock.0.x));

        for (_, group) in &rocks.iter_mut().group_by(|rock| rock.0.y) {
            let mut low_x = 0;
            for (coords, rock_type) in group {
                match rock_type {
                    Rock::Cube => {
                        low_x = coords.x + 1;
                    }
                    Rock::Round => {
                        coords.x = low_x;
                        low_x += 1;
                    }
                };
            }
        }
    }

    fn shift_east(&mut self) {
        let Self { rocks, ref bounds } = self;
        rocks.sort_by_key(|rock| (rock.0.y, Reverse(rock.0.x)));

        for (_, group) in &rocks.iter_mut().group_by(|rock| rock.0.y) {
            let mut high_x = bounds - 1;
            for (coords, rock_type) in group {
                match rock_type {
                    Rock::Cube => {
                        high_x = coords.x.saturating_sub(1);
                    }
                    Rock::Round => {
                        coords.x = high_x;
                        high_x = high_x.saturating_sub(1);
                    }
                };
            }
        }
    }

    fn get_north_load(&self) -> usize {
        let Self { rocks, bounds } = self;
        rocks
            .iter()
            .map(|(coords, rock_type)| match rock_type {
                Rock::Cube => 0,
                Rock::Round => bounds - coords.y,
            })
            .sum()
    }

    fn cycle(&mut self) {
        self.shift_north();
        self.shift_west();
        self.shift_south();
        self.shift_east();
    }
}

impl Iterator for Rocks {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.cycle();
        Some(self.get_north_load())
    }
}

pub fn main() {
    let input = &include_bytes!("../../input/day14")[..];
    let rocks = Rocks::from_bytes(&input);

    let p1 = {
        let mut rocks = rocks.clone();
        rocks.shift_north();
        rocks.get_north_load()
    };
    println!("part 1: {p1}");

    let p2 = {
        let rocks = rocks.clone();
        let loads = rocks.into_iter().cycle().take(200).collect_vec();

        let first_cycle_val = {
            let mut load_frequency: IndexMap<usize, u8> = IndexMap::new();

            for load in loads.iter() {
                load_frequency
                    .entry(*load)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }

            load_frequency
                .iter()
                .find(|(_, freq)| **freq > 3)
                .unwrap()
                .0
                .to_owned()
        };

        let (cycle_start, cycle_end) = loads
            .iter()
            .positions(|load| *load == first_cycle_val)
            .take(2)
            .collect_tuple()
            .unwrap();

        let cycle = &loads[cycle_start..cycle_end];
        cycle[(1000000000 - (cycle_start + 1)) % cycle.len()]
    };
    println!("part 2: {p2}");
}
