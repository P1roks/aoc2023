#[derive(Debug)]
struct Race {
    time: usize,
    min_distance: usize,
}

impl Race {
    fn get_win_number(&self) -> usize {
        let delta: f64 = ((self.time * self.time - 4 * self.min_distance) as f64).sqrt();
        let min_win = ((self.time as f64 - delta) / 2f64).ceil() as usize;
        let max_win = ((self.time as f64 + delta) / 2f64).floor() as usize;

        max_win - min_win + 1
    }
}

fn parse_input_part1(input: &str) -> Vec<Race> {
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .zip(input.lines().nth(1).unwrap().split_whitespace().skip(1))
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            min_distance: distance.parse().unwrap(),
        })
        .collect()
}

fn parse_input_part2(input: &str) -> Race {
    let time = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .filter(|chr| chr.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let min_distance = input
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .filter(|chr| chr.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    Race { time, min_distance }
}

pub fn main() {
    let input = include_str!("../../input/day06");
    let parsed_part1 = parse_input_part1(input);
    let part1 = parsed_part1
        .iter()
        .map(|race| race.get_win_number())
        .product::<usize>();
    println!("part 1: {part1}");

    let parsed_part2 = parse_input_part2(input);
    let part2 = parsed_part2.get_win_number();
    println!("part 2: {part2}");
}
