use std::{error::Error, ops::Range};

#[derive(Debug, PartialEq, Eq)]
struct ConvertMap {
    dest_start: i64,
    source_start: i64,
    range: i64,
}

impl ConvertMap {
    fn from_string(source: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<_> = source.split(' ').collect();
        if parts.len() != 3 {
            Err("Wrong Input! Expected: <dest_start> <source_start> <range>".into())
        } else {
            Ok(Self {
                dest_start: parts[0].parse()?,
                source_start: parts[1].parse()?,
                range: parts[2].parse()?,
            })
        }
    }

    fn convert(&self, source: i64) -> Option<i64> {
        let source_diff = source - self.source_start;
        if source_diff >= 0 && source_diff < self.range {
            Some(self.dest_start + source_diff)
        } else {
            None
        }
    }
}

fn parse_seeds(seeds: &str) -> Vec<i64> {
    seeds
        .split(' ')
        .skip(1)
        .map(|no| no.parse().unwrap())
        .collect()
}

fn parse_seeds_ranges(seeds: &str) -> Vec<Range<i64>> {
    seeds
        .split(' ')
        .skip(1)
        .step_by(2)
        .zip(seeds.split(' ').skip(2).step_by(2))
        .map(|(no, range)| {
            let no: i64 = no.parse().unwrap();
            let range: i64 = range.parse().unwrap();
            no..(no + range)
        })
        .collect()
}

fn get_maps(map: &str) -> Result<Vec<ConvertMap>, Box<dyn Error>> {
    map.lines().skip(1).map(ConvertMap::from_string).collect()
}

fn seed_location(mut source: i64, categories: &[Vec<ConvertMap>]) -> i64 {
    for category in categories {
        let valid = category
            .iter()
            .skip_while(|map| map.convert(source).is_none())
            .next();
        if let Some(map) = valid {
            source = map.convert(source).unwrap();
        }
    }
    source
}

fn part1(seeds: &[i64], categories: &[Vec<ConvertMap>]) -> i64 {
    seeds
        .iter()
        .map(|seed| seed_location(*seed, categories))
        .min()
        .unwrap_or(0)
}

fn part2(seeds_range: &[Range<i64>], categories: &[Vec<ConvertMap>]) -> i64 {
    let mut location = i64::MAX;
    for range in seeds_range.into_iter() {
        location = std::cmp::min(
            range
                .clone()
                .map(|seed| seed_location(seed, categories))
                .min()
                .unwrap_or(i64::MAX),
            location,
        );
    }
    location
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() {
        let convert = ConvertMap::from_string("50 98 2").unwrap();
        let valid = ConvertMap {
            dest_start: 50,
            source_start: 98,
            range: 2,
        };

        assert_eq!(convert, valid);
    }

    #[test]
    fn test_convert() {
        let convert_map = ConvertMap {
            dest_start: 50,
            source_start: 98,
            range: 2,
        };
        assert_eq!(convert_map.convert(98), Some(50));
        assert_eq!(convert_map.convert(99), Some(51));
    }

    #[test]
    fn test_convert_advanced() {
        let convert_map = ConvertMap {
            dest_start: 52,
            source_start: 50,
            range: 48,
        };
        assert_eq!(convert_map.convert(53), Some(55));
        assert_eq!(convert_map.convert(49), None);
    }
}

fn main() {
    let input = include_str!("./input");
    let seeds = parse_seeds(input.lines().next().unwrap());
    let seeds_ranges = parse_seeds_ranges(input.lines().next().unwrap());
    let categories: Vec<_> = input
        .split("\n\n")
        .skip(1)
        .map(get_maps)
        .map(|map| map.unwrap())
        .collect();

    let part2 = part2(&seeds_ranges, &categories);
    println!("part 2: {part2}");
}
