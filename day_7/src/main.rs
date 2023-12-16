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
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('1', 1),
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
       // println!("Card: {:?}", card);

        cards.push(card);
    }

    cards
}