use std::{fs::File, io::{BufReader, BufRead, Lines}};

#[derive(Debug)]
struct Race {
    pub time: u64,
    pub dist: u64,
    win_hold_times: Vec<u64>,
}

impl Race {
    fn new(time: u64, dist: u64) -> Self {
        Self { 
            time,
            dist,
            win_hold_times: vec![]
        }
    }

    fn add_win_hold_time(&mut self, hold_time: u64) {
        self.win_hold_times.push(hold_time);
    }

    fn win_hold_times(&self) -> usize {
        self.win_hold_times.len()
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut race = parse_race(reader);
    let wins = calc_margin(&mut race);

    println!("Wins: {}", wins);
}

fn calc_margin(race: &mut Race) -> usize {
    for hold_time in 1..race.time {
        let remaining_time = race.time - hold_time;
        let distance = calc_distance(hold_time, remaining_time);
        
        if distance > race.dist {
            race.add_win_hold_time(hold_time);
        }
    }

    race.win_hold_times()
}

fn calc_distance(hold_time: u64, run_time: u64) -> u64 {
    hold_time * run_time
}

fn parse_race(reader: BufReader<File>) -> Race {
    let mut lines = reader.lines();
    let time = parse_line(&mut lines);
    let dist = parse_line(&mut lines);

    Race::new(time, dist)
}

fn parse_line(lines: &mut Lines<BufReader<File>>) -> u64 {
    let num_line: String = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect();

    println!("Num str: {}", num_line);

    num_line.parse::<u64>().unwrap()
}