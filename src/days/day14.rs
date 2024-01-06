use itertools::Itertools;

const MAX_LOAD: usize = 100usize;

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Rock {
    Round,
    Cube,
}

fn get_rocks(positions: &&[u8]) -> Vec<(Coords, Rock)> {
    let mut rocks = Vec::new();
    for (y, line) in positions.split(|p| *p == b'\n').enumerate() {
        for (x, chr) in line.iter().enumerate() {
            match chr {
                b'O' => rocks.push((Coords { x, y }, Rock::Round)),
                b'#' => rocks.push((Coords { x, y }, Rock::Cube)),
                _ => {}
            };
        }
    }

    rocks
}

fn shift_north(rocks: &mut Vec<(Coords, Rock)>) {
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

fn get_north_load(rocks: Vec<(Coords, Rock)>) -> usize {
    rocks
        .iter()
        .map(|(coords, rock_type)| match rock_type {
            Rock::Cube => 0,
            Rock::Round => MAX_LOAD - coords.y,
        })
        .sum()
}

pub fn main() {
    let input = &include_bytes!("../../input/day14")[..];
    let mut rocks = get_rocks(&input);
    shift_north(&mut rocks);
    let p1 = get_north_load(rocks);
    println!("part 1: {p1}");
}
