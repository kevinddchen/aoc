use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Finds the error character in a rucksack
fn find_rucksack_error(rucksack: &String) -> char {
    // keep hashset of characters in the first compartment
    let mut first_compartment: HashSet<char> = HashSet::new();

    assert!(rucksack.len() % 2 == 0);
    for (i, c) in rucksack.chars().enumerate() {
        if i < rucksack.len() / 2 {
            // first compartment
            first_compartment.insert(c);
        } else {
            // second compartment
            if first_compartment.contains(&c) {
                // found a match
                return c;
            }
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

fn main() {
    let mut total_priority = 0;

    // read lines from the file one-by-one
    let file = File::open("data/day3.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let error = find_rucksack_error(&line);
        total_priority += get_priority(error);
    }

    println!("Total priority: {}", total_priority);
}
