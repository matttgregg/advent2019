use std::time::SystemTime;

use itertools::Itertools;

use advent2019::intcode::{Computer, IntState};

pub fn run() {
    println!("Day7!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data7.txt");
    let contents = String::from_utf8_lossy(cbytes);

    try_combinations(&contents);
    try_combinations_multi(&contents);

    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("Timed: {}us", timed);
}

fn try_combinations(code: &str) {
    let mut best_val = 0;
    let mut best_combo = vec![0,0,0,0,0];
    for combination in (0..5).permutations(5) {
        let res = run_once(code, &combination);
        if res > best_val {
            best_val = res;
            best_combo = combination;
        }
    }

    println!("BEST:{:?} -> {}", best_combo, best_val);
}

fn try_combinations_multi(code: &str) {
    let mut best_val = 0;
    let mut best_combo = vec![0,0,0,0,0];
    for combination in (5..10).permutations(5) {
        let res = run_many(code, &combination);
        if res > best_val {
            best_val = res;
            best_combo = combination;
        }
    }

    println!("BEST-FEEDBACK:{:?} -> {}", best_combo, best_val);
}

fn run_many(code: &str, inits: &[i64]) -> i64 {
    // Initialize computers
    let mut computers: Vec<Computer> = Vec::new();
    for i in inits {
        let mut  computer = Computer::new(code);
        computer.input.push_back(*i);
        computers.push(computer);
    }

    let mut outputs = vec![0]; // There's an initial 0 given.
    let mut terminated = false;

    while !terminated {
        for mut c in &mut computers {
            if c.state == IntState::Finished {
                // We break once we have reached a finished computer again.
                terminated = true;
                break;
            }
            for o in outputs {
                c.input.push_back(o);
            }
            outputs = run_to_output(&mut c);
        }
    }
    outputs[0] // We return the final output.
}

fn run_once(code: &str, inits: &[i64]) -> i64 {
    // Initialize computers
    let mut computers: Vec<Computer> = Vec::new();
    for i in inits {
        let mut  computer = Computer::new(code);
        computer.input.push_back(*i);
        computers.push(computer);
    }

    let mut outputs = vec![0]; // There's an initial 0 given.
    for mut c in computers {
        for o in outputs {
            c.input.push_back(o);
        }
        outputs = run_to_output(&mut c);
    }
    outputs[0] // We return the final output.
}

fn run_to_output(computer: &mut Computer) -> Vec<i64> {
    // Single push to waiting computers.
    if computer.state == IntState::Waiting {
        computer.run();
    }
    while computer.state == IntState::Ready {
        computer.run();
    }

    // Collect output
    let mut outp:Vec<i64> = Vec::new();
    while let Some(v) = computer.output.pop_front() {
        outp.push(v);
    }
    outp
}
