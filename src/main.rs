use std::{env, time::Instant};

mod days;
use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day13, day14,
    day15, day16, day19,
};

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Usage: cargo r -- <day>")
        .parse::<u8>()
        .unwrap_or(0);

    let time = Instant::now();
    match day {
        1 => day01::main(),
        2 => day02::main(),
        3 => day03::main(),
        4 => day04::main(),
        5 => day05::main(),
        6 => day06::main(),
        7 => day07::main(),
        8 => day08::main(),
        9 => day09::main(),
        10 => day10::main(),
        11 => day11::main(),
        13 => day13::main(),
        14 => day14::main(),
        15 => day15::main(),
        16 => day16::main(),
        19 => day19::main(),
        _ => unimplemented!("No more day for now!"),
    };
    println!("elapsed: {}s", time.elapsed().as_secs_f64());
}
