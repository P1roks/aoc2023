use std::collections::HashMap;

const CARD_OFFSET: usize = 10;

fn point_worth(line: &str) -> Option<u32> {
    let (winning, chosen) = line[CARD_OFFSET..].split_once('|')?;

    let mut winning: HashMap<&str, u8> = winning.split_whitespace().map(|no| (no, 0)).collect();
    chosen.split_whitespace().for_each(|no| {
        winning.entry(no).and_modify(|count| *count += 1);
    });

    let sum: u32 = winning.into_values().sum::<u8>() as u32;
    if sum == 0 {
        None
    } else {
        Some(sum)
    }
}

fn part2(input: &str) -> u32 {
    let mut count: HashMap<usize, u32> = input.lines().enumerate().map(|(no, _)| (no, 1)).collect();

    for (idx, worth) in input
        .lines()
        .map(|line| point_worth(line).unwrap_or(0))
        .enumerate()
    {
        let card_count = count.get(&idx).unwrap().to_owned();

        ((idx + 1)..=(idx + worth as usize)).for_each(|idx| {
            count.entry(idx).and_modify(|count| *count += card_count);
        });
    }

    count.into_values().sum::<u32>()
}

fn main() {
    let input = include_str!("./input");
    let part1 = input
        .lines()
        .filter_map(point_worth)
        .map(|number| 2u32.pow(number - 1))
        .sum::<u32>();
    println!("part1: {part1}");
    println!("part2: {}", part2(input));
}
