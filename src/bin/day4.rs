use std::fs::File;
use std::io::{BufRead, BufReader};

struct Assignment {
    start: i32,
    end: i32,
}

/// Given two assignments, determine if one is contained within the other
fn contained(a: &Assignment, b: &Assignment) -> bool {
    let a_in_b = a.start >= b.start && a.end <= b.end;
    let b_in_a = b.start >= a.start && b.end <= a.end;
    return a_in_b || b_in_a;
}

/// Given two assignments, determine if they overlap
fn overlap(a: &Assignment, b: &Assignment) -> bool {
    let a_before_b = a.end < b.start;
    let b_before_a = b.end < a.start;
    return !a_before_b && !b_before_a;
}

/// Given a string slice "123-456", parse it into an Assignment struct
fn parse_assignment(part: &str) -> Assignment {
    let ends: Vec<&str> = part.split('-').collect();
    assert!(ends.len() == 2, "Could not parse ends for part: {}", part);
    let start: i32 = ends[0].parse().expect("Could not parse start");
    let end: i32 = ends[1].parse().expect("Could not parse end");
    return Assignment {
        start,
        end,
    };
}

/// Given a line "123-456,789-012", parse it into two Assignment structs
fn parse_assignments(line: &String) -> (Assignment, Assignment) {
    let parts: Vec<&str> = line.split(',').collect();
    assert!(parts.len() == 2, "Could not parse parts for line: {}", line);
    return (parse_assignment(parts[0]), parse_assignment(parts[1]));
}

fn main() {
    let mut contained_count = 0;
    let mut overlap_count = 0;

    // read lines from the file one-by-one
    let file = File::open("data/day4.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        // parse the line into two assignments
        let (a, b) = parse_assignments(&line);
        if contained(&a, &b) {
            contained_count += 1;
        }
        if overlap(&a, &b) {
            overlap_count += 1;
        }
    }

    println!("Contained assignments: {}", contained_count);
    println!("Overlapping assignments: {}", overlap_count);
}
