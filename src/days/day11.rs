use core::fmt;
use std::collections::HashMap;

use itertools::Itertools;

struct Map(Vec<Vec<u8>>);

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn get_distance(&self, other: &Coords) -> usize {
        let x = {
            if self.x > other.x {
                self.x - other.x
            } else {
                other.x - self.x
            }
        };
        let y = {
            if self.y > other.y {
                self.y - other.y
            } else {
                other.y - self.y
            }
        };
        x + y
    }
}

impl Map {
    fn new(lines: &[u8]) -> Self {
        Self(
            lines
                .split(|num| *num == b'\n')
                .map(Vec::from)
                .collect_vec(),
        )
    }

    fn get_empty(&self) -> (Vec<usize>, Vec<usize>) {
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut cols_map = HashMap::new();

        for (row_idx, line) in self.0.iter().enumerate().take(self.0.len() - 1) {
            let mut empty_row = true;
            for (col_idx, chr) in line.iter().enumerate() {
                if *chr == b'#' {
                    empty_row = false;
                    cols_map.insert(col_idx, true);
                }
            }
            if empty_row {
                rows.push(row_idx);
            }
        }

        for idx in 0..(self.0.len() - 1) {
            if !cols_map.contains_key(&idx) {
                cols.push(idx);
            }
        }

        (cols, rows)
    }

    fn get_galaxies_expanded(&self, expand_dist: usize) -> Vec<Coords> {
        let (cols, rows) = self.get_empty();
        let mut expand_y = 0;
        let mut galaxies = Vec::new();

        for (y, line) in self.0.iter().enumerate() {
            if rows.contains(&y) {
                expand_y += expand_dist;
            }
            let mut expand_x = 0;
            for (x, char) in line.iter().enumerate() {
                if cols.contains(&x) {
                    expand_x += expand_dist;
                }
                if *char == b'#' {
                    galaxies.push(Coords {
                        x: x + expand_x,
                        y: y + expand_y,
                    });
                }
            }
        }
        galaxies
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut txt = String::new();
        for line in self.0.iter() {
            for chr in line {
                txt.push(*chr as char);
            }
            txt.push('\n');
        }
        write!(f, "{}", txt)
    }
}

fn solve_for_expand_dist(map: &Map, expand: usize) -> usize {
    let mut sum: usize = 0;
    let galaxies = map.get_galaxies_expanded(expand);

    for (idx, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies[(idx + 1)..galaxies.len()].iter() {
            sum += galaxy1.get_distance(galaxy2);
        }
    }
    sum
}

pub fn main() {
    let input = include_bytes!("../../input/day11");
    let map = Map::new(input);

    println!("part1: {}", solve_for_expand_dist(&map, 1));
    println!("part2: {}", solve_for_expand_dist(&map, 999_999));
}
