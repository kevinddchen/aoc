use std::fs::File;
use std::io::{BufRead, BufReader};

/// Returns the total calories held by each elf.
fn get_total_calories() -> Vec<i32> {

    let mut total_calories: Vec<i32> = Vec::new();  // calories for each elf
    let mut running_total = 0;  // tracks calories as we add up

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
            total_calories.push(running_total);
            running_total = 0;
        } else {
            // parse line
            let calories = line.parse::<i32>().expect("Could not parse line");
            running_total += calories;
        }
    }

    // at end of file, append final running total
    total_calories.push(running_total);

    return total_calories;

}

fn main() {

    let mut total_calories = get_total_calories();

    // sort the calories; the last element is the max
    total_calories.sort();

    println!("Max calories: {}", total_calories.last().unwrap());
    println!("Sum of top three calories: {}", total_calories.iter().rev().take(3).sum::<i32>());

}
