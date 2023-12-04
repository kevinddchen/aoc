// File containing puzzle data
const DATA: &str = "data/22day1.txt";

/// Returns the calories held by each elf
fn get_calories() -> Vec<i32> {
    let mut calories: Vec<i32> = Vec::new(); // calories for each elf
    let mut running_sum = 0; // running sum of calories for the current elf

    // iterate over lines of the file
    let data = std::fs::read_to_string(DATA).unwrap();

    for line in data.split("\n") {
        if line.len() == 0 {
            // encountered empty line
            calories.push(running_sum);
            running_sum = 0;
        } else {
            // parse line
            let calories: i32 = line.parse().unwrap();
            running_sum += calories;
        }
    }

    // at end of file, append final running sum
    if running_sum > 0 {
        calories.push(running_sum);
    }

    return calories;
}

fn main() {
    let mut calories = get_calories();

    // sort the calories; the last element is the max
    // NOTE: this is asymptotically too slow, but easy to implement and fast enough for this problem
    calories.sort();

    let max = calories.last().unwrap();
    println!("Part 1: {}", max); // Expected output: 71023

    let top_three = &calories[(calories.len() - 3)..];
    let top_three_sum: i32 = top_three.iter().sum();
    println!("Part 2: {}", top_three_sum); // Expected output: 206289
}
