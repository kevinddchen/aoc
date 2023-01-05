use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Finds the error character in a rucksack
fn find_rucksack_error(rucksack: &String) -> char {
    // keep set of characters in the first compartment
    let mut first_compartment: HashSet<char> = HashSet::new();

    assert!(rucksack.len() % 2 == 0);
    let half_len = rucksack.len() / 2;

    // first compartment
    for c in rucksack[..half_len].chars() {
        first_compartment.insert(c);
    }

    // look for a match in the second compartment
    for c in rucksack[half_len..].chars() {
        if first_compartment.contains(&c) {
            return c;
        }
    }
    panic!("Could not find rucksack error");
}

/// Get the priority of a character
fn get_priority(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as i32) - ('a' as i32) + 1;
    } else {
        return (c as i32) - ('A' as i32) + 27;
    }
}

/// Solve first half of the puzzle
fn error_total_priority() -> i32 {
    let mut total_priority = 0;

    // read lines from the file one-by-one
    let file = File::open("data/day3.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let error = find_rucksack_error(&line);
        total_priority += get_priority(error);
    }

    return total_priority;
}

/// Solve second half of the puzzle
fn badge_total_priority() -> i32 {
    let mut total_priority = 0;

    // read lines from the file one-by-one
    let file = File::open("data/day3.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut all_items: HashSet<char> = HashSet::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Could not read line");

        if i % 3 == 0 {
            // first line of a group
            all_items = HashSet::new();
            for c in line.chars() {
                all_items.insert(c);
            }
        } else {
            // subsequent line of a group
            let mut new_items: HashSet<char> = HashSet::new();
            for c in line.chars() {
                if all_items.contains(&c) {
                    new_items.insert(c);
                }
            }
            all_items = new_items;
        }

        if i % 3 == 2 {
            // last line of a group
            assert!(all_items.len() == 1);
            for c in all_items.iter() {
                total_priority += get_priority(*c);
            }
        }
    }

    return total_priority;
}

fn main() {
    let error_total_priority = error_total_priority();
    println!("Error total priority: {}", error_total_priority);

    let badge_total_priority = badge_total_priority();
    println!("Badge total priority: {}", badge_total_priority);
}
