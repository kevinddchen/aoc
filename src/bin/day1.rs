use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let mut max = 0;  // maximum calories for an elf so far
    let mut running_total = 0;  // calories for each elf

    // read lines from the file one-by-one
    let file = File::open("data/day1.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        
        if line.len() == 0 {
            // encountered newline
            if running_total == 0 {
                panic!("Did not expect zero running total");
            }
            max = std::cmp::max(max, running_total);
            running_total = 0;
        } else {
            // parse line
            let calories = line.parse::<i32>().expect("Could not parse line");
            running_total += calories;
        }
    }

    // at end of file, do final check
    max = std::cmp::max(max, running_total);

    println!("Max calories: {}", max);

}
