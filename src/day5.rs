use std::time::SystemTime;

use advent2019::intcode::{Computer, IntState};

pub fn run() {
    println!("Day5!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data5.txt");
    let contents = String::from_utf8_lossy(cbytes);


    let res1 = run_to_output(&contents, 1);
    let res2 = run_to_output(&contents, 5);
    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("Diagnostic code, system 1 = {}", res1);
    println!("Diagnostic code, system 5 = {}", res2);
    println!("Timed: {}us", timed);
}

fn run_to_output(code: &str, id: i64) -> i64 {
    let mut computer = Computer::new(code);
    computer.input.push_back(id);
    while computer.state == IntState::Ready {
        computer.run();
    }

    let mut try_val = computer.output.pop_front();
    let mut val = 0;
    while let Some(x) = try_val {
        val = x;
        try_val = computer.output.pop_front();
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_tests() {
        assert_eq!(1, run_to_output("3,9,8,9,10,9,4,9,99,-1,8", 8));
        assert_eq!(0, run_to_output("3,9,8,9,10,9,4,9,99,-1,8", 7));
        assert_eq!(1, run_to_output("3,9,7,9,10,9,4,9,99,-1,8",7));
        assert_eq!(0, run_to_output("3,9,7,9,10,9,4,9,99,-1,8",9));
        assert_eq!(1, run_to_output("3,3,1108,-1,8,3,4,3,99", 8));
        assert_eq!(0, run_to_output("3,3,1108,-1,8,3,4,3,99", 7));
        assert_eq!(1, run_to_output("3,3,1107,-1,8,3,4,3,99", 7));
        assert_eq!(0, run_to_output("3,3,1107,-1,8,3,4,3,99", 9));
        // assert_eq!(, run_to_output("",));
    }

    #[test]
    fn jump_tests() {
        assert_eq!(0, run_to_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0));
        assert_eq!(1, run_to_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 2));
        assert_eq!(0, run_to_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0));
        assert_eq!(1, run_to_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 2));
    }

    #[test]
    fn large_test() {
        let code = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(999, run_to_output(&code, 3));
        assert_eq!(1000, run_to_output(&code, 8));
        assert_eq!(1001, run_to_output(&code, 24));
    }
} 
