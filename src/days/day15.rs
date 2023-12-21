use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
enum Operation {
    Remove,
    Replace(u8),
}

#[derive(Debug)]
struct Step {
    label: &'static [u8],
    operation: Operation,
}

impl Step {
    fn from_seq(seq: &&'static [u8]) -> Self {
        if seq.iter().last().unwrap().is_ascii_digit() {
            Self {
                label: &seq[0..seq.len() - 2],
                operation: Operation::Replace(*seq.iter().last().unwrap() - 48),
            }
        } else {
            Self {
                label: &seq[0..seq.len() - 1],
                operation: Operation::Remove,
            }
        }
    }
}

fn get_hash(vals: &&[u8]) -> usize {
    let mut curr = 0usize;

    for ascii in vals.iter() {
        curr += *ascii as usize;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn calculate_hash<T: Hash>(t: &T) -> usize {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() as usize
}

fn part2(steps: Vec<&'static [u8]>) -> usize {
    // cannot do [IndexMap::new(); 256], bcs IndexMap doesn't implement std::Marker::Copy
    let mut boxes: [IndexMap<usize, u8>; 256] = (0..256)
        .map(|_| IndexMap::new())
        .collect_vec()
        .try_into()
        .unwrap();

    let steps = steps.iter().map(Step::from_seq).collect_vec();

    for step in steps.iter() {
        let r#box = &mut boxes[get_hash(&step.label)];
        let hash = calculate_hash(&step.label);
        match step.operation {
            Operation::Remove => {
                r#box.shift_remove(&hash);
            }
            Operation::Replace(focal) => {
                r#box
                    .entry(hash)
                    .and_modify(|x| *x = focal)
                    .or_insert(focal);
            }
        };
    }

    let mut sum = 0;
    for (box_idx, r#box) in boxes.iter().enumerate() {
        for (item_idx, item) in r#box.values().enumerate() {
            sum += (box_idx + 1) * (item_idx + 1) * (*item as usize);
        }
    }

    sum
}

pub fn main() {
    let input = include_bytes!("../../input/day15");
    let input: Vec<&[u8]> = input[0..input.len() - 1]
        .split(|s| *s == b',')
        .collect_vec();

    let p1 = input.iter().map(get_hash).sum::<usize>();
    println!("part 1: {p1}");

    let p2 = part2(input);
    println!("part 2: {p2}");
}
