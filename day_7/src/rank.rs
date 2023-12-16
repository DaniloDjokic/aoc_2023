use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Rank {
    Kind(u32),
    FullHouse,
    Pair(u32),
    High,
}

impl From<&str> for Rank {
    fn from(value: &str) -> Self {
        let mut map: HashMap<char, usize> = HashMap::new();

        for char in value.chars() {
            match map.get_mut(&char) {
                Some(count) => *count += 1,
                None => _ = map.insert(char, 1)
            }
        }

        let mut vals: Vec<usize> = map.values().map(|k| *k).collect();

        vals.sort();
        vals.reverse();

       // println!("Card: {}, keys: {:?}", value, vals);

        return match &vals[0] {
            5 => Rank::Kind(5),
            4 => Rank::Kind(4),
            3 => {
                match &vals[1] {
                    2 => Rank::FullHouse,
                    1 => Rank::Kind(3),
                    _ => panic!("Found 3 but next was not 2 or 1")
                }
            },
            2 => {
                match &vals[1] {
                    2 => Rank::Pair(2),
                    1 => Rank::Pair(1),
                    _ => panic!("Found 2 but next was not 2 or 1")
                }
            },
            1 => Rank::High,
            _ => panic!("Key is not betwwen 1 and 5")
        }
    }
}