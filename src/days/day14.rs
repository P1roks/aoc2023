use itertools::Itertools;

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

fn part1(positions: &&[u8]) -> usize {
    let max_load = 100usize;
    let mut rocks = get_rocks(positions);
    let mut sum = 0;
    rocks.sort_by_key(|rock| (rock.0.x, rock.0.y));

    for (_, group) in &rocks.iter().group_by(|rock| rock.0.x) {
        let mut low_y = 0;
        for (coords, rock_type) in group {
            match rock_type {
                Rock::Cube => {
                    low_y = coords.y + 1;
                }
                Rock::Round => {
                    sum += max_load - low_y;
                    low_y += 1;
                }
            };
        }
    }

    sum
}

pub fn main() {
    let input = &include_bytes!("../../input/day14")[..];
    let p1 = part1(&input);
    println!("part 1: {p1}");
}
