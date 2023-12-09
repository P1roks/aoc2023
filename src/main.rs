mod days;
use std::env;

use days::{day01, day02, day03, day04, day05, day06, day07, day08};

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Usage: cargo r -- <day>")
        .parse::<u8>()
        .unwrap_or(0);

    match day {
        1 => day01::main(),
        2 => day02::main(),
        3 => day03::main(),
        4 => day04::main(),
        5 => day05::main(),
        6 => day06::main(),
        7 => day07::main(),
        8 => day08::main(),
        _ => unimplemented!("No more day for now!"),
    };
}
