use std::time::SystemTime;

use advent2019::intcode::{Computer, IntState};

pub fn run() {
    println!("Day2!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data2.txt");
    let contents = String::from_utf8_lossy(cbytes);


    let res1 = run_noun_verb(&contents, 12, 2);
    let (n, v) = search_for(&contents, 19690720);
    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("Final result for  12, 2 = {}", res1);
    println!("Correct values for moon orbit: ({}, {}) => {}", n, v, 100*n + v);
    println!("Timed: {}us", timed);
}

fn run_noun_verb(code: &str, noun: i64, verb: i64) -> i64 {
    let mut computer =  Computer::new(code);
    computer.poke(1, noun);
    computer.poke(2, verb);
    while computer.state == IntState::Ready {
        computer.run();
    }
    computer.peek(0)
}

fn search_for(code: &str, target: i64) -> (i64, i64) {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_noun_verb(code, noun, verb) == target {
                return (noun, verb);
            }
        }
    }

    (-1, -1)
}
