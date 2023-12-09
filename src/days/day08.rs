use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

#[derive(Hash, Debug)]
struct Entry {
    left: u64,
    right: u64,
    ghost_status: GhostStatus,
}

#[derive(Hash, Debug, PartialEq)]
enum GhostStatus {
    StartNode,
    EndNode,
    Neither,
}

enum Directions {
    Left,
    Right,
}

impl TryFrom<char> for Directions {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Only L and R are valid directions"),
        }
    }
}

fn calculate_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn parse_entry(entry: &str) -> (u64, Entry) {
    let (name, directions) = entry.split_once(" = ").unwrap();

    let (left, right) = directions.split_once(", ").unwrap();

    let left = &left[1..];
    let right = &right[..right.len() - 1];

    let ghost_status = if name.ends_with('A') {
        GhostStatus::StartNode
    } else if name.ends_with('Z') {
        GhostStatus::EndNode
    } else {
        GhostStatus::Neither
    };

    let entry = Entry {
        left: calculate_hash(left),
        right: calculate_hash(right),
        ghost_status,
    };
    let name = calculate_hash(name);

    (name, entry)
}

fn part1(directions: &[Directions], entries: &HashMap<u64, Entry>) -> usize {
    let endpoint = calculate_hash("ZZZ");
    let mut curr_point = calculate_hash("AAA");

    for (step, direction) in directions.iter().cycle().enumerate() {
        curr_point = {
            let entry = entries.get(&curr_point).unwrap();
            match direction {
                Directions::Left => entry.left,
                Directions::Right => entry.right,
            }
        };

        if curr_point == endpoint {
            return step + 1;
        }
    }
    unreachable!()
}

fn part2(directions: &[Directions], entries: &HashMap<u64, Entry>, mut curr_point: u64) -> usize {
    for (step, direction) in directions.iter().cycle().enumerate() {
        curr_point = {
            let entry = entries.get(&curr_point).unwrap();
            if entry.ghost_status == GhostStatus::EndNode {
                return step;
            }

            match direction {
                Directions::Left => entry.left,
                Directions::Right => entry.right,
            }
        };
    }
    unreachable!();
}

pub fn main() {
    let input = include_str!("../../input/day08");
    let directions = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|chr| Directions::try_from(chr).unwrap())
        .collect_vec();
    let entries: HashMap<u64, Entry> = input.lines().skip(2).map(parse_entry).collect();
    let p1 = part1(&directions, &entries);
    println!("part 1: {p1}");

    let ghost_start_nodes = entries
        .iter()
        .filter(|(_, val)| val.ghost_status == GhostStatus::StartNode)
        .map(|(key, _)| *key)
        .collect_vec();
    let p2 = ghost_start_nodes
        .iter()
        .map(|node| part2(&directions, &entries, *node))
        .fold(1, num::integer::lcm);

    println!("part 2: {p2}");
}
