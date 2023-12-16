use crate::rank::Rank;

use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct Hand<'a> {
    pub card_line: String,
    pub bid: u64,
    rank: Rank,
    cmp_map: &'a HashMap<char, u32>,
}

impl<'a> Hand<'a> {
    pub fn new(card_line: &str, rank: Rank, bid: &str, cmp_map: &'a HashMap<char, u32>) -> Self {
        Self {
            card_line: String::from(card_line),
            bid: bid.parse::<u64>().unwrap(),
            rank: rank,
            cmp_map: cmp_map,
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("Comparing {} to {}", self.card_line, other.card_line);
        return match self.rank {
            Rank::Kind(5) => match other.rank {
                Rank::Kind(5) => self.line_cmp(other),
                _ => Ordering::Greater
            },
            Rank::Kind(4) => match other.rank {
                Rank::Kind(5) => Ordering::Less,
                Rank::Kind(4) => self.line_cmp(other),
                _ => Ordering::Greater
            },
            Rank::FullHouse => match other.rank {
                Rank::Kind(5) | Rank::Kind(4) => Ordering::Less,
                Rank::FullHouse => self.line_cmp(other),
                _ => Ordering::Greater
            },
            Rank::Kind(3) => match other.rank {
                Rank::Kind(5) | Rank::Kind(4) | Rank::FullHouse => Ordering::Less,
                Rank::Kind(3) => self.line_cmp(other),
                _ => Ordering::Greater
            },
            Rank::Pair(2) => match other.rank {
                Rank::Pair(1) | Rank::High => Ordering::Greater,
                Rank::Pair(2) => self.line_cmp(other),
                _ => Ordering::Less
            },
            Rank::Pair(1) => match  other.rank {
                Rank::High => Ordering::Greater,
                Rank::Pair(1) => self.line_cmp(other),
                _ => Ordering::Less
            },
            Rank::High => match other.rank {
                Rank::High => self.line_cmp(other),
                _ => Ordering::Less
            },
            _ => panic!("What is this rank?")
        }
    }
}

impl<'a> Hand<'a> {
    fn line_cmp(&self, other: &Self) -> Ordering {
        for (self_char, other_char) in self.card_line.chars().zip(other.card_line.chars()) {
            if self_char == other_char {
                continue;
            }

            return self.cmp_map.get(&self_char).unwrap().cmp(self.cmp_map.get(&other_char).unwrap());
        }

        panic!("Can't compare {} to {}", self.card_line, other.card_line);
    }
}