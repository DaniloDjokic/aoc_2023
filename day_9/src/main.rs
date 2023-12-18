use std::{fs::File, io::{BufReader, BufRead }};

#[derive(Debug)]
struct ValueHistory {
    history: Vec<i64>,
    derivatives: Vec<Vec<i64>>
}

impl From<&str> for ValueHistory {
    fn from(value: &str) -> Self {
        let history = value.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
        let derivatives = ValueHistory::calc_derivatives(&history);

        Self { history, derivatives }
    }
}

impl ValueHistory {
    fn calc_next(&mut self, last: bool) {
        let last_vec = &self.derivatives[self.derivatives.len() - 1];
        let mut prev;

        if last {
            prev = last_vec[last_vec.len() - 1];
        }
        else {
            prev = last_vec[0];
        }

        //println!("Last vec {:?}, last el {}", last_vec, prev);

        for i in (0..=self.derivatives.len()-1).rev() {
            let curr_vec = &mut self.derivatives[i];
            let curr;
            if last {
                curr = curr_vec[curr_vec.len() - 1];
            }
            else {
                curr = curr_vec[0];
            }

            //println!("Looking at {:?}, last el {}", curr_vec, curr);

            if last {
                curr_vec.push(prev + curr);
                prev = *curr_vec.last().unwrap(); 
            }
            else {
                curr_vec.insert(0, curr - prev);
                prev = *curr_vec.first().unwrap();
            }
        }

        if last {
            self.history.push(self.history.last().unwrap() + self.derivatives[0].last().unwrap());
        }
        else {
            self.history.insert(0, self.history.first().unwrap() - self.derivatives[0].first().unwrap());
        }
    }

    fn calc_derivatives(history: &Vec<i64>) -> Vec<Vec<i64>> {
        //toooooooo much cloning
        //try to find a more performant way to do this

        let mut col = vec![];

        let mut curr_line = history.clone();
        let mut new_line = curr_line.clone(); 

        while !new_line.iter().all(|x| *x == 0) {
            new_line.clear();

            for pairs in curr_line.windows(2) {
                new_line.push(pairs[1] - pairs[0]);
            }
            
            col.push(new_line.clone());
            curr_line = new_line.clone();
        }
        
        col
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let histories = parse_file(reader);

    let val = histories.iter().fold(0, |acc, h| {
        acc + h.history.first().unwrap()
    });

    println!("Val {val}");
}

fn parse_file(reader: BufReader<File>) -> Vec<ValueHistory> {
    let mut histories =vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut history = ValueHistory::from(&line[..]);
        history.calc_next(true);
        history.calc_next(false);
        histories.push(history);
    }

    histories
}