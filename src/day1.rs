use std::time::SystemTime;

pub fn run() {
    println!("Day1!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data1.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let (total, adjusted) = run_string(&contents);

    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("Fuel requirments: {} raw, {} adjusted", total, adjusted);
    println!("Timed: {}us", timed);
}

fn fuel_for(v: usize) -> usize {
    if v <= 6 {
        0
    } else {
        (v / 3) - 2
    }
}

fn adjusted_fuel_for(v: usize) -> usize {
    let mut total = fuel_for(v);
    let mut addition = total;
    while addition > 0 {
        addition = fuel_for(addition);
        total += addition;
    };

    total
}

fn run_string(contents: &str) -> (usize, usize) {
    let mut total_fuel = 0;
    let mut total_adjusted = 0;
    for l in contents.lines().filter_map(|x| x.parse::<usize>().ok()) {
        total_fuel += fuel_for(l);
        total_adjusted += adjusted_fuel_for(l);
    }

    (total_fuel, total_adjusted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let cbytes = include_bytes!("../data/data1.txt");
        let contents = String::from_utf8_lossy(cbytes);
        assert_eq!((3318604, 4975039), run_string(&contents));
    }

    #[test]
    fn test_examples() {
        assert_eq!(2, fuel_for(12));
        assert_eq!(2, fuel_for(14));
        assert_eq!(654, fuel_for(1969));
        assert_eq!(33583, fuel_for(100756));
    }

    #[test]
    fn test_adjusted() {
        assert_eq!(2, adjusted_fuel_for(14));
        assert_eq!(966, adjusted_fuel_for(1969));
        assert_eq!(50346, adjusted_fuel_for(100756));
    }
}
