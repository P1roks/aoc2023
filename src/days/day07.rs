use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Rank {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    // Jack,
    Tower,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Jack,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Tower),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("Wrong input!"),
        }
    }
}

#[derive(Debug)]
struct Draw {
    rank: Rank,
    cards: Vec<Card>,
    bid: usize,
}

impl Draw {
    #[allow(dead_code)]
    fn get_rank_frequency_part1(frequency: HashMap<char, u8>) -> Rank {
        let mut max_val = 1;
        for val in frequency.into_values() {
            match (val, max_val) {
                (2, 2) => return Rank::TwoPair,
                (3, 2) | (2, 3) => return Rank::FullHouse,
                _ => {}
            };

            max_val = std::cmp::max(max_val, val);
        }
        match max_val {
            1 => Rank::HighCard,
            2 => Rank::OnePair,
            3 => Rank::ThreeOfKind,
            4 => Rank::FourOfKind,
            5 => Rank::FiveOfKind,
            _ => unreachable!(),
        }
    }

    fn get_rank_frequency_part2(mut frequency: HashMap<char, u8>) -> Rank {
        let mut max_val = 0;
        let jokers = frequency.remove(&'J').unwrap_or(0);
        for val in frequency.into_values() {
            match (val, max_val) {
                (2, 2) => {
                    if jokers == 0 {
                        return Rank::TwoPair;
                    } else {
                        return Rank::FullHouse;
                    }
                }
                (3, 2) | (2, 3) => return Rank::FullHouse,
                _ => {}
            };

            max_val = std::cmp::max(max_val, val);
        }
        max_val += jokers;
        match max_val {
            1 => Rank::HighCard,
            2 => Rank::OnePair,
            3 => Rank::ThreeOfKind,
            4 => Rank::FourOfKind,
            5 => Rank::FiveOfKind,
            _ => unreachable!(),
        }
    }

    fn parse_draw(draw: &str) -> Self {
        let (cards_txt, bid) = draw.split_once(' ').unwrap();
        let mut cards: Vec<Card> = Vec::with_capacity(cards_txt.len());
        let mut frequency: HashMap<char, u8> = HashMap::new();

        for card in cards_txt.chars() {
            cards.push(card.try_into().unwrap());
            frequency
                .entry(card)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }

        let rank = Self::get_rank_frequency_part2(frequency);

        Self {
            rank,
            cards,
            bid: bid.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_card_parse() {
        assert_eq!(Rank::FullHouse, Draw::parse_draw("AJJAA 1").rank);
        assert_eq!(Rank::HighCard, Draw::parse_draw("23456 1").rank);
        assert_eq!(Rank::TwoPair, Draw::parse_draw("2JJAA 1").rank);
        assert_eq!(Rank::FiveOfKind, Draw::parse_draw("AAAAA 1").rank);
        assert_eq!(Rank::FourOfKind, Draw::parse_draw("QQJQQ 1").rank);
        assert_eq!(Rank::ThreeOfKind, Draw::parse_draw("A23AA 1").rank);
    }
}

pub fn main() {
    let input = include_str!("../../input/day07");
    let mut parsed = input.lines().map(Draw::parse_draw).collect_vec();
    parsed.sort_by_key(|draw| (draw.rank, draw.cards.clone()));
    let part = parsed
        .iter()
        .rev()
        .enumerate()
        .map(|(val, draw)| (val + 1) * draw.bid)
        .sum::<usize>();
    println!("part: {part}");
}
