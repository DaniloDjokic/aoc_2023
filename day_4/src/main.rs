use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

#[derive(Clone, Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    current: HashSet<u32>,
    copy_count: u32,
}

impl Card {
    fn get_winnings(&self) -> usize {
        let common = self.winning.intersection(&self.current);

        common.into_iter().count()
    }

    fn new() -> Self {
        Self { id: 0, winning: HashSet::new(), current: HashSet::new(), copy_count: 1 }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut cards = parse_file(reader);
    cards = append_cards(cards);

    for i in 0..cards.len() {
       // println!("Card: {}, copies {}", cards[i].id, cards[i].copy_count);
    }

    println!("Total length: {}", cards.iter().fold(0, |acc, c| {
        acc + c.copy_count
    }));
}

fn append_cards(mut cards: Vec<Card>) -> Vec<Card> {
    let starting_len = cards.len();
    let mut copies: Vec<Card> = vec![];

    for i in 0..cards.len()  {
        for _j in 0..cards[i].copy_count {
            let current_card = &cards[i];
            let win_val = current_card.get_winnings();

            let bound = if i + win_val > starting_len {starting_len} else {i + win_val};

            let to_copy = &mut cards[i+1..=bound];

            for copy in to_copy {
             //   println!("Updating: {}", copy.id);
                copy.copy_count += 1;
            } 
        }
    }
    
    cards.append(&mut copies);

    cards
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
