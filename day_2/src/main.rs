use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub cubes: HashMap<Cube, u32>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cube {
    Red,
    Green,
    Blue,
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let games = parse_file(reader);
    let value = games.iter().fold(0, |acc, g| {
        acc + g.cubes.iter().fold(1, |acc, c| {
            acc * *c.1
        })
    });

    println!("The value is {}", value);
}

fn parse_file(reader: BufReader<File>) -> Vec<Game> {    
    let mut games = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game = parse_game(&line);
                games.push(game);
            },
            Err(e) => {
                eprintln!("Cannot read line, {}", e);
            }
        }
    }

    games
}

fn parse_game(line: &str) -> Game {
    let id_start_index = line.find(" ").unwrap();
    let colon_index = line.find(':').unwrap();
    let game_id = &line[id_start_index+1..colon_index];

    let game_id = game_id.parse::<u32>().unwrap_or(0);

    let game_strings: Vec<&str> = line[colon_index+1..].split(";").collect();

    let mut game = Game {
        id: game_id,
        cubes: HashMap::from([
            (Cube::Red, 0),
            (Cube::Green, 0),
            (Cube::Blue, 0)
        ]),
    };

    for game_str in game_strings {
        let cubes: Vec<&str> = game_str.split(',').collect();
        
        for cube_str in cubes {
            let cube_str = cube_str.trim();
            let whitespace_idx = cube_str.find(" ").unwrap();
            
            let cube_count = &cube_str[..whitespace_idx]
                .parse::<u32>()
                .unwrap();

            let cube_color = &cube_str.trim()[whitespace_idx+1..];

            let new_cube = match cube_color {
                "blue" => Cube::Blue,
                "red" => Cube::Red,
                "green" => Cube::Green,
                _ => panic!("Cannot parse cube {}", cube_str)
            };

            let current_count = game.cubes.get(&new_cube).unwrap();

            if cube_count > current_count {
                game.cubes.insert(new_cube, *cube_count);
            }
        }
    }

    game
}