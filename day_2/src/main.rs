use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub cubes: Vec<Cube>,
}

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let games = parse_file(reader);
    let value = games.iter().fold(0, |acc, g| {
        acc + g.id
    });

    for game in games {
        println!("{:?}", game);
    }

    println!("The value is {}", value);
}

fn parse_file(reader: BufReader<File>) -> Vec<Game> {    
    let mut games = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game = parse_game(&line);

                if !game.cubes
                    .iter()
                    .any(|c| {
                        match c {
                            Cube::Red(v) => *v > 12,
                            Cube::Green(v) => *v > 13,
                            Cube::Blue(v) => *v > 14
                        }  
                    }) {
                        games.push(game);
                    }
            },
            Err(e) => {
                eprintln!("Cannot read line");
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
        cubes: vec![],
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

            let cube = match cube_color {
                "blue" => Cube::Blue(*cube_count),
                "red" => Cube::Red(*cube_count),
                "green" => Cube::Green(*cube_count),
                _ => panic!("Cannot parse cube {}", cube_str)
            };

            game.cubes.push(cube);
        }
    }

    game
}