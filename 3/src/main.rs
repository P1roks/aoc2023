use itertools::{Itertools, Position};

fn part_numbers_sum(lines: &[&str]) -> u64 {
    let mut sum: u64 = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut last_idx: usize = 0;
        let mut group_no: usize = 0;
        for (_, group) in &line
            .chars()
            .enumerate()
            .filter(|(_, chr)| chr.is_ascii_digit())
            .map(|(x, chr)| (x, chr.to_digit(10).unwrap() as u64))
            .group_by(|(x, _)| {
                let same_group = *x == 0 || (x - 1) == last_idx;
                last_idx = *x;
                if !same_group {
                    group_no += 1;
                }
                group_no
            })
        {
            let mut is_valid = false;
            let mut number = 0;
            for (pos, (x, no)) in group.with_position() {
                number *= 10;
                number += no;
                if !is_valid {
                    let left = x.saturating_sub(1);
                    let top = y.saturating_sub(1);
                    match pos {
                        Position::First => {
                            is_valid |= is_symbol(line.chars().nth(left)) // left
                                || is_symbol(lines.get(y + 1).and_then(|line| line.chars().nth(left))) // left-bottom
                                || is_symbol(lines.get(top).and_then(|line| line.chars().nth(left)));
                            // left-top
                        }
                        Position::Last => {
                            is_valid |= is_symbol(line.chars().nth(x + 1)) // right
                                || is_symbol(lines.get(y + 1).and_then(|line| line.chars().nth(x + 1))) // right-bottom
                                || is_symbol(lines.get(top).and_then(|line| line.chars().nth(x + 1)));
                            // right-top
                        }
                        Position::Only => {
                            is_valid |= is_symbol(line.chars().nth(left)) // left
                                || is_symbol(lines.get(y + 1).and_then(|line| line.chars().nth(left))) // left-bottom
                                || is_symbol(lines.get(top).and_then(|line| line.chars().nth(left))) // left-top
                                || is_symbol(line.chars().nth(x + 1)) // right
                                || is_symbol(lines.get(y + 1).and_then(|line| line.chars().nth(x + 1))) // right-bottom
                                || is_symbol(lines.get(top).and_then(|line| line.chars().nth(x + 1)));
                            // right-top
                        }
                        _ => {}
                    };
                    is_valid |= is_symbol(lines.get(y + 1).and_then(|line| line.chars().nth(x))) // bottom
                    || is_symbol(lines.get(top).and_then(|line| line.chars().nth(x)));
                    // top
                }
                if pos == Position::Last || pos == Position::Only {
                    if is_valid {
                        sum += number;
                    }
                    number = 0;
                    is_valid = false;
                }
            }
        }
    }

    sum
}

fn is_symbol(chr: Option<char>) -> bool {
    if let Some(chr) = chr {
        !chr.is_ascii_digit() && chr != '.'
    } else {
        false
    }
}

fn get_number(line: Option<&&str>, x: usize) -> Option<u64> {
    let line = line?;
    if line.chars().nth(x).unwrap_or('.').is_ascii_digit() {
        let mut leftmost = x;
        let mut number: u64 = 0;
        while leftmost != 0
            && line
                .chars()
                .nth(leftmost - 1)
                .unwrap_or('.')
                .is_ascii_digit()
        {
            leftmost -= 1;
        }
        for idx in leftmost..(leftmost + 3) {
            if let Some(digit) = line.chars().nth(idx).unwrap_or('.').to_digit(10) {
                number *= 10;
                number += digit as u64;
            } else {
                break;
            }
        }

        Some(number)
    } else {
        None
    }
}

fn adjacent_numbers(schematic: &[&str], x: usize, y: usize) -> Vec<u64> {
    let mut numbers = Vec::new();
    let top = y.saturating_sub(1);
    let left = x.saturating_sub(1);

    // If only Rust has is_some_and...

    let mut get_push_number = |y: usize, x: usize| -> bool {
        if let Some(number) = get_number(schematic.get(y), x) {
            numbers.push(number);
            true
        } else {
            false
        }
    };

    if !get_push_number(top, x) {
        get_push_number(top, x + 1);
        get_push_number(top, left);
    }
    if !get_push_number(y + 1, x) {
        get_push_number(y + 1, x + 1);
        get_push_number(y + 1, left);
    }

    get_push_number(y, left);
    get_push_number(y, x + 1);

    numbers
}

fn gears_sum(schematic: &[&str], gear: char) -> u64 {
    let mut sum = 0;

    for (y, line) in schematic.iter().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, chr)| *chr == gear) {
            let adj = adjacent_numbers(schematic, x, y);
            if adj.len() == 2 {
                sum += adj.iter().product::<u64>();
            }
        }
    }

    sum
}

fn main() {
    let input: Vec<_> = include_str!("./input").split('\n').collect();
    println!("part 1: {}", part_numbers_sum(&input));
    println!("part 2: {}", gears_sum(&input, '*'));
}
