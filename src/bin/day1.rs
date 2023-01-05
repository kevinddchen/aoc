use std::fs::File;
use std::io::{BufRead, BufReader};

/// Returns the calories held by each elf.
fn get_each_calories() -> Vec<i32> {
    let mut each_calories: Vec<i32> = Vec::new(); // calories for each elf
    let mut running_sum = 0; // tracks calories as we add up

    // read lines from the file one-by-one
    let file = File::open("data/day1.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        if line.len() == 0 {
            // encountered newline
            each_calories.push(running_sum);
            running_sum = 0;
        } else {
            // parse line
            let calories: i32 = line.parse().expect("Could not parse line");
            running_sum += calories;
        }
    }

    // at end of file, append final running total
    each_calories.push(running_sum);

    return each_calories;
}

fn main() {
    let mut total_calories = get_each_calories();

    // sort the calories; the last element is the max
    total_calories.sort();

    let max_calories = total_calories.last().unwrap();
    println!("Max calories: {}", max_calories);

    let top_three_sum: i32 = total_calories.iter().rev().take(3).sum();
    println!("Sum of top three calories: {}", top_three_sum);
}
