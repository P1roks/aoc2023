use std::cmp::max;
struct Cube {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Cube {
    fn satisfies_part1(&self) -> bool {
        if self.red <= 12 && self.green <= 13 && self.blue <= 14 {
            true
        } else {
            false
        }
    }

    fn get_power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn new(id: u32) -> Self {
        Self {
            id,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn parse_line(line: &str, no: u32) -> Cube {
    let mut cube = Cube::new(no);
    for (number, mut color) in line
        .split(' ')
        .skip(2)
        .step_by(2)
        .zip(line.split(' ').skip(3).step_by(2))
    {
        let number = number.parse::<u32>().unwrap();

        if color.ends_with(';') || color.ends_with(',') {
            color = &color[0..color.len() - 1];
        }

        match color {
            "red" => cube.red = max(cube.red, number),
            "green" => cube.green = max(cube.green, number),
            "blue" => cube.blue = max(cube.blue, number),
            err @ _ => unreachable!(
                "Invalid color supplied! Valid colors are red, green, blue. Supplied value: {err}"
            ),
        }
    }

    cube
}

pub fn main() {
    let input = include_str!("../../input/day02");

    let cubes = input
        .lines()
        .enumerate()
        .map(|(no, line)| parse_line(line, (no + 1) as u32))
        .collect::<Vec<_>>();

    let part1_ans = cubes
        .iter()
        .filter(|cube| cube.satisfies_part1())
        .map(|cube| cube.id)
        .sum::<u32>();

    let part2_ans = cubes.iter().map(|cube| cube.get_power()).sum::<u32>();

    println!("part 1: {part1_ans} part 2: {part2_ans}");
}
