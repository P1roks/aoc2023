use std::{collections::HashMap, ops::Sub};

use itertools::Itertools;

trait ToNum {
    fn to_number(&self) -> u32;
}

impl ToNum for &[u8] {
    fn to_number(&self) -> u32 {
        let mut number = 0u32;
        for byte in self.iter() {
            number *= 10;
            number += byte.sub(48) as u32;
        }
        number
    }
}

impl ToNum for &[&u8] {
    fn to_number(&self) -> u32 {
        let mut number = 0u32;
        for byte in self.iter() {
            number *= 10;
            number += byte.sub(48) as u32;
        }
        number
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn from_bytes(input: &[u8]) -> Option<Self> {
        if input.len() == 0 {
            return None;
        }
        let input = &input[1..(input.len() - 1)];
        let [x, m, a, s, ..] = input.split(|byte| *byte == b',').collect_vec()[..] else {
            return None;
        };

        Some(Self {
            x: (&x[2..]).to_number(),
            m: (&m[2..]).to_number(),
            a: (&a[2..]).to_number(),
            s: (&s[2..]).to_number(),
        })
    }

    fn sum(&self) -> usize {
        (self.x + self.m + self.a + self.s) as usize
    }
}

#[derive(Debug)]
enum Symbol {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum RuleResult {
    Accept,
    Reject,
    Check(String),
}

impl RuleResult {
    fn from_bytes(bytes: &[u8]) -> Self {
        match bytes {
            [b'R', ..] => Self::Reject,
            [b'A', ..] => Self::Accept,
            _ => Self::Check(String::from_utf8_lossy(bytes).to_string()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    symbol: Symbol,
    less_than: bool,
    value: u32,
    accept: RuleResult,
}

impl Rule {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut it = bytes.iter();
        let symbol = match it.next() {
            Some(b'x') => Symbol::X,
            Some(b'm') => Symbol::M,
            Some(b'a') => Symbol::A,
            Some(b's') => Symbol::S,
            _ => panic!(),
        };
        let less_than = {
            if let Some(symbol) = it.next() {
                *symbol == b'<'
            } else {
                false
            }
        };
        let value = it
            .take_while(|byte| **byte != b':')
            .collect_vec()
            .as_slice()
            .to_number();
        let accept = RuleResult::from_bytes(bytes.split(|byte| *byte == b':').last().unwrap());

        Self {
            symbol,
            less_than,
            value,
            accept,
        }
    }

    fn is_satisfied_by(&self, part: &Part) -> bool {
        match (&self.symbol, self.less_than) {
            (Symbol::X, true) => part.x < self.value,
            (Symbol::X, false) => part.x > self.value,
            (Symbol::M, true) => part.m < self.value,
            (Symbol::M, false) => part.m > self.value,
            (Symbol::A, true) => part.a < self.value,
            (Symbol::A, false) => part.a > self.value,
            (Symbol::S, true) => part.s < self.value,
            (Symbol::S, false) => part.s > self.value,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    tests: Vec<Rule>,
    default: RuleResult,
}

impl Workflow {
    fn from_bytes(input: &[u8]) -> Self {
        let mut split = input.split(|byte| *byte == b',').rev();
        let default = RuleResult::from_bytes(split.next().unwrap());
        let tests = split.map(Rule::from_bytes).rev().collect_vec();

        Self { tests, default }
    }

    fn test_part<'a>(&'a self, part: &Part) -> &'a RuleResult {
        if let Some(idx) = self
            .tests
            .iter()
            .position(|test| test.is_satisfied_by(part))
        {
            &self.tests[idx].accept
        } else {
            &self.default
        }
    }
}

fn solve_part1(parts: &Vec<Part>, workflows: &HashMap<String, Workflow>) -> usize {
    parts.iter().fold(0, |acc, part| {
        let mut workflow = "in";
        loop {
            match workflows[workflow].test_part(part) {
                RuleResult::Accept => {
                    return acc + part.sum();
                }
                RuleResult::Reject => {
                    return acc;
                }
                RuleResult::Check(val) => {
                    workflow = val.as_str();
                }
            }
        }
    })
}

pub fn main() {
    let input = include_str!("../../input/day19");
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let parts = parts
        .as_bytes()
        .split(|bytes| *bytes == b'\n')
        .filter_map(Part::from_bytes)
        .collect_vec();

    let workflows: HashMap<String, Workflow> = workflows
        .as_bytes()
        .split(|bytes| *bytes == b'\n')
        .map(|full| {
            let id: String = full
                .iter()
                .take_while(|byte| **byte != b'{')
                .map(|val| *val as char)
                .collect();

            let workflow = Workflow::from_bytes(
                full.iter()
                    .skip(id.len() + 1)
                    .take_while(|byte| **byte != b'}')
                    .map(|val| val.to_owned())
                    .collect_vec()
                    .as_slice(),
            );
            (id, workflow)
        })
        .collect();

    let p1 = solve_part1(&parts, &workflows);
    println!("part1: {p1}");
}
