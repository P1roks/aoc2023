fn first_last_dig(line: &str) -> u32 {
    let mut first_it = line.chars();
    let mut last_it = line.chars().rev();
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    fn advance_number<T>(iterator: &mut T) -> Option<u32>
    where
        T: Iterator<Item = char>,
    {
        if let Some(number) = iterator.next() {
            if number.is_digit(10) {
                return number.to_digit(10);
            }
            return None;
        }
        None
    }

    // Basic two pointers approach
    while first.is_none() || last.is_none() {
        if first.is_none() {
            first = advance_number(&mut first_it);
        }
        if last.is_none() {
            last = advance_number(&mut last_it);
        }
    }
    first.unwrap() * 10 + last.unwrap()
}

pub fn main() {
    let mut input: String = String::from(include_str!("../../input/day01"));

    // Really bad but this problem is mid so w/e
    let part2 = true;
    if part2 {
        input = input
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
    }
    let ans = input.lines().map(first_last_dig).sum::<u32>();
    println!("{ans}");
}
