use itertools::Itertools;

fn get_line_byte(values: &&[u8]) -> u32 {
    values.iter().enumerate().fold(0, |acc, (idx, value)| {
        if *value == b'#' {
            acc | (1 << idx)
        } else {
            acc
        }
    })
}

fn get_reflection(hashed: &Vec<u32>) -> Option<usize> {
    'outer: for (mut left, mut right) in (0..hashed.len()).tuple_windows() {
        let mirror_point = left + 1;
        while hashed[left] == hashed[right] {
            if left == 0 || right == (hashed.len() - 1) {
                return Some(mirror_point);
            }
            left -= 1;
            right += 1;
            if right == hashed.len() {
                break 'outer;
            }
        }
    }

    None
}

fn hash_vertical(ground: &Vec<&[u8]>) -> Vec<u32> {
    let mut cols: Vec<Vec<u8>> = vec![Vec::with_capacity(ground.len()); ground[0].len()];

    for line in ground {
        for (idx, val) in line.iter().enumerate() {
            cols[idx].push(*val);
        }
    }

    cols.iter()
        .map(|single| get_line_byte(&single.as_slice()))
        .collect_vec()
}

pub fn main() {
    let patterns = include_str!("../../input/day13")
        .split("\n\n")
        .collect_vec();

    let p1 = patterns.iter().fold(0, |acc, ground| {
        let ground = ground
            .as_bytes()
            .split(|x| *x == b'\n')
            .take_while(|v| !v.is_empty())
            .collect_vec();

        let hashed_horizontal = ground.iter().map(get_line_byte).collect_vec();
        if let Some(val) = get_reflection(&hashed_horizontal) {
            return acc + val * 100;
        }
        let hashed_vertical = hash_vertical(&ground);
        if let Some(val) = get_reflection(&hashed_vertical) {
            return acc + val;
        }
        acc
    });

    println!("part1: {p1}");
}
