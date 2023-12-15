use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
struct Race {
    pub id: usize,
    pub time: u32,
    pub dist: u32,
    win_hold_times: Vec<u32>,
}

impl Race {
    fn new(id: usize) -> Self {
        Self { id, time: 0, dist: 0, win_hold_times: vec![] }
    }

    fn add_win_hold_time(&mut self, hold_time: u32) {
        self.win_hold_times.push(hold_time);
    }

    fn win_hold_times(&self) -> usize {
        self.win_hold_times.len()
    }
}

enum RaceValue {
    Time,
    Distance
}

impl RaceValue {
    fn get_value_idx(idx: usize) -> Self {
        match idx {
            0 => RaceValue::Time,
            1 => RaceValue::Distance,
            _ => panic!("Unexpected index: {}", idx)
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut races = parse_races(reader);
    let margin = calc_margin(&mut races);

    println!("Margin: {}", margin);
}

fn calc_margin(races: &mut Vec<Race>) -> usize {
    let mut total_margin = 1;

    for race in races.iter_mut() {
        for hold_time in 1..race.time {
            let remaining_time = race.time - hold_time;
            let distance = calc_distance(hold_time, remaining_time);
            
            if distance > race.dist {
                race.add_win_hold_time(hold_time);
            }
        }

        total_margin *= race.win_hold_times();
    }

    total_margin
}

fn calc_distance(hold_time: u32, run_time: u32) -> u32 {
    hold_time * run_time
}

fn parse_races(reader: BufReader<File>) -> Vec<Race> {
   let mut races: Vec<Race> = vec![];

    for (value_idx, line) in reader.lines().enumerate() {
        for (race_idx, value) in line
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .to_owned()
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .enumerate()
        {
            match races.iter_mut().find(|r| r.id == race_idx) {
                Some(race) => {
                    match RaceValue::get_value_idx(value_idx) {
                        RaceValue::Time => race.time = value,
                        RaceValue::Distance => race.dist = value
                    }
                },
                None => {
                    let mut race = Race::new(race_idx);

                    match RaceValue::get_value_idx(value_idx) {
                        RaceValue::Time => race.time = value,
                        RaceValue::Distance => race.dist = value
                    }
        
                    races.push(race);
                }
            }
            
        }
    }

    races
}