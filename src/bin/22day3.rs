use std::collections::HashSet;

// File containing puzzle data
const DATA: &str = "data/22day3.txt";

/// Read rucksacks from the text file
fn get_rucksacks() -> Vec<String> {
    let data = std::fs::read_to_string(DATA).unwrap();
    let rucksacks: Vec<String> = data.split("\n").map(|s| s.to_string()).collect();
    return rucksacks;
}

/// Finds the error character in a rucksack
fn find_rucksack_error(rucksack: &str) -> char {
    assert!(rucksack.len() % 2 == 0, "Rucksack length is not even");

    let half_len = rucksack.len() / 2;
    let first_compartment = rucksack[..half_len].to_string();
    let second_compartment = rucksack[half_len..].to_string();

    // keep set of characters in the first compartment
    let mut set: HashSet<char> = HashSet::new();

    for c in first_compartment.chars() {
        set.insert(c);
    }

    for c in second_compartment.chars() {
        if set.contains(&c) {
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

fn find_badge(rucksacks: (&str, &str, &str)) -> char {
    // find an element that is in all three rucksacks

    let mut set0: HashSet<char> = HashSet::new();
    for c in rucksacks.0.chars() {
        set0.insert(c);
    }

    let mut set1: HashSet<char> = HashSet::new();
    for c in rucksacks.1.chars() {
        if set0.contains(&c) {
            set1.insert(c);
        }
    }

    for c in rucksacks.2.chars() {
        if set1.contains(&c) {
            return c;
        }
    }

    panic!("Could not find badge");
}

fn main() {
    let rucksacks = get_rucksacks();

    // Part 1
    {
        let mut total_priority = 0;
        for rucksack in &rucksacks {
            let error = find_rucksack_error(rucksack);
            total_priority += get_priority(error);
        }
        println!("Part 1: {}", total_priority); // Expected output: 7831
    }

    // Part 2
    {
        assert!(
            rucksacks.len() % 3 == 0,
            "Expected number of rucksacks to be divisible by 3"
        );

        let mut badge_total_priority = 0;
        for i in 0..(rucksacks.len() / 3) {
            let first_compartment = &rucksacks[3 * i];
            let second_compartment = &rucksacks[3 * i + 1];
            let third_compartment = &rucksacks[3 * i + 2];
            let badge = find_badge((first_compartment, second_compartment, third_compartment));
            badge_total_priority += get_priority(badge);
        }
        println!("Part 2: {}", badge_total_priority); // Expected output: 2683
    }
}
