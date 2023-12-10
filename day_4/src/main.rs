use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

struct Card {
    id: u32,
    winning: HashSet<u32>,
    current: HashSet<u32>,
}

impl Card {
    fn get_value(&self) -> u32 {
        let common = self.winning.intersection(&self.current);

        let count = common.into_iter().count();

        match count {
            0 => 0,
            _ => 2_u32.pow((count - 1) as u32)
        }
    }

    fn new() -> Self {
        Self { id: 0, winning: HashSet::new(), current: HashSet::new() }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let cards = parse_file(reader);

    for i in 0..cards.len() {
        println!("Card: {}, Value: {}", cards[i].id, cards[i].get_value());
    }

    let value = cards.iter().fold(0, |acc, c| {
        acc + c.get_value()
    });

    println!("The value is: {}", value);
}

fn parse_file(reader: BufReader<File>) -> Vec<Card> {
    let mut cards = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let lines: Vec<&str> = line.split(':').collect();

        let id = lines[0]
            .trim()
            .split("Card ")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        let num_collection: Vec<&str> = lines[1].split('|').collect();

        let mut card = Card::new();

        card.id = id;

        for num in num_collection[0].split(" ") {
            if let Ok(num) = num.trim().parse::<u32>() {
                card.winning.insert(num);
            }
        }

        for num in num_collection[1].split(" ") {
            if let Ok(num) = num.trim().parse::<u32>() {
                card.current.insert(num);
            }
        }

        cards.push(card);
    }

    cards
}
