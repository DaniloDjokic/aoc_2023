use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use hand::Hand;

mod rank;
mod hand;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let cmp_map = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 11),
        ('9', 10),
        ('8', 9),
        ('7', 8),
        ('6', 7),
        ('5', 6),
        ('4', 5),
        ('3', 4),
        ('2', 3),
        ('1', 2),
        ('J', 1),
    ]);

    let mut hands = parse_file(reader, &cmp_map);
    hands.sort();

    let val = hands.iter().enumerate().fold(0, |acc, (i,h)| {
        let mul = (i + 1) as u64;
        acc + h.bid * mul
    });

    println!("Val: {}", val);
}

fn parse_file(reader: BufReader<File>, cmp_map: &HashMap<char, u32>) -> Vec<Hand> {
    let mut cards = vec![];
    

    for line in reader.lines() {
        let line_u = line.unwrap();
        let (card_line, bid) = line_u.split_once(" ").unwrap().to_owned();

        let rank = rank::Rank::from(card_line);
        let card = Hand::new(card_line, rank, bid, cmp_map);
        //println!("Card: {}, rank: {:?}", card.card_line, card.rank);

        cards.push(card);
    }

    cards
}