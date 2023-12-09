use itertools::Itertools;

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect_vec()
}

fn part1(history: Vec<i64>) -> i64 {
    let changes = history
        .iter()
        .tuple_windows()
        .map(|(first, second)| second - first)
        .collect_vec();

    if changes.iter().all(|change| *change == 0) {
        return *history.iter().last().unwrap();
    }

    history.iter().last().unwrap() + part1(changes)
}

fn part2(history: Vec<i64>) -> i64 {
    let changes = history
        .iter()
        .tuple_windows()
        .map(|(first, second)| second - first)
        .collect_vec();

    if changes.iter().all(|change| *change == 0) {
        return *history.iter().next().unwrap();
    }

    history.iter().next().unwrap() - part2(changes)
}

pub fn main() {
    let input = include_str!("../../input/day09");
    let mut lines = input.lines().map(parse_line).collect_vec();
    let p1 = lines.clone().drain(..).map(part1).sum::<i64>();
    println!("part 1: {p1}");

    let p2 = lines.drain(..).map(part2).sum::<i64>();
    println!("part 2: {p2}");
}
