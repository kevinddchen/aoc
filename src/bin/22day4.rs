// File containing puzzle data
const DATA: &str = "data/22day4.txt";

#[derive(Debug)]
struct Assignment {
    start: i32,
    end: i32,
}

/// Read assignments from the text file
fn get_assignments() -> Vec<(Assignment, Assignment)> {
    let mut assignments: Vec<(Assignment, Assignment)> = Vec::new();

    let data = std::fs::read_to_string(DATA).unwrap();
    for line in data.split("\n") {
        let mut iter = line.split(',');
        let first_assignment = parse_assignment(iter.next().unwrap());
        let second_assignment = parse_assignment(iter.next().unwrap());
        assignments.push((first_assignment, second_assignment));
    }

    return assignments;
}

/// Given a string like "123-456", parse it into an `Assignment``
fn parse_assignment(s: &str) -> Assignment {
    let mut iter = s.split('-');
    let start: i32 = iter.next().unwrap().parse().unwrap();
    let end: i32 = iter.next().unwrap().parse().unwrap();
    return Assignment { start, end };
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

fn main() {
    let assignments = get_assignments();

    // Part 1
    {
        let mut contained_count = 0;
        for (a, b) in &assignments {
            if contained(a, b) {
                contained_count += 1;
            }
        }
        println!("Part 1: {}", contained_count); // Expected output: 475
    }

    // Part 2
    {
        let mut overlap_count = 0;
        for (a, b) in &assignments {
            if overlap(a, b) {
                overlap_count += 1;
            }
        }
        println!("Part 2: {}", overlap_count); // Expected output: 825
    }
}
