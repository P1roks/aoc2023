use itertools::Itertools;

trait ByteVal {
    // returns true when value is a value which a single bit can take in a byte
    // e.g. 1,2,4,8,16,2048...
    fn is_byte_val(&self) -> bool;
}
impl ByteVal for usize {
    fn is_byte_val(&self) -> bool {
        *self != 0 && (self & (self - 1)) == 0
    }
}

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
    for (mut left, mut right) in (0..hashed.len()).tuple_windows() {
        let mirror_point = left + 1;
        while hashed[left] == hashed[right] {
            if left == 0 || right == (hashed.len() - 1) {
                return Some(mirror_point);
            }
            left -= 1;
            right += 1;
        }
    }

    None
}

fn get_reflection_part_two(hashed: &Vec<u32>) -> Option<usize> {
    for (mut left, mut right) in (0..hashed.len()).tuple_windows() {
        let mut smudge = false;
        let mirror_point = left + 1;
        loop {
            if hashed[left] != hashed[right] {
                if smudge {
                    break;
                }

                let diff = {
                    let left = hashed[left];
                    let right = hashed[right];
                    if left > right {
                        left - right
                    } else {
                        right - left
                    }
                } as usize;

                if !diff.is_byte_val() {
                    break;
                }
                smudge = true;
            }

            if left == 0 || right == (hashed.len() - 1) {
                if smudge {
                    return Some(mirror_point);
                }
                break;
            }
            left -= 1;
            right += 1;
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

    fn ground_bytes<'a>(ground: &'a &str) -> Vec<&'a [u8]> {
        ground
            .as_bytes()
            .split(|x| *x == b'\n')
            .take_while(|v| !v.is_empty())
            .collect_vec()
    }

    let p1 = patterns.iter().fold(0, |acc, ground| {
        let ground = ground_bytes(ground);
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

    let p2 = patterns.iter().fold(0, |acc, ground| {
        let ground = ground_bytes(ground);
        // Vertical has priority for some reason and in part 2 both vertical and horizontal can be
        // true so evaluating vertical first is a must here
        let hashed_vertical = hash_vertical(&ground);
        if let Some(val) = get_reflection_part_two(&hashed_vertical) {
            return acc + val;
        }
        let hashed_horizontal = ground.iter().map(get_line_byte).collect_vec();
        if let Some(val) = get_reflection_part_two(&hashed_horizontal) {
            return acc + val * 100;
        }
        acc
    });
    println!("part2: {p2}");
}
