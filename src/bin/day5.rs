use std::fs::File;
use std::io::{BufRead, BufReader};

struct Move {
    num: i32,
    source: usize,
    dest: usize,
}

/// Hard-coded initial crate stack configuration
fn initialize_stacks() -> [Vec<char>; 9] {
    [
        vec!['H', 'C', 'R'],
        vec!['B', 'J', 'H', 'L', 'S', 'F'],
        vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],
        vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],
        vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'],
        vec!['T', 'H', 'C', 'G'],
        vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'],
        vec!['R', 'J', 'Q', 'G', 'C'],
        vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S'],
    ]
}

/// Concatenates the characters at the top of each stack into a string
fn stack_tops(stacks: &[Vec<char>; 9]) -> String {
    let mut result = String::new();
    for stack in stacks.iter() {
        let last = match stack.last() {
            Some(last) => last,
            None => &' ',
        };
        result.push(*last);
    }
    return result;
}

/// Parse move from a line of the file
fn parse_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split(' ').collect();
    let num: i32 = parts[1].parse().expect("Could not parse num");
    let source: usize = parts[3].parse::<usize>().expect("Could not parse source") - 1;
    let dest: usize = parts[5].parse::<usize>().expect("Could not parse dest") - 1;
    return Move {
        num,
        source,
        dest,
    };
}

/// Execute the move on the stacks, moving one-by-one
fn execute_move_part1(m: &Move, stacks: &mut [Vec<char>; 9]) {
    for _ in 0..m.num {
        let c = stacks[m.source].pop().expect("Could not pop from source");
        stacks[m.dest].push(c);
    }
}

/// Execute the move on the stacks, moving all at once
fn execute_move_part2(m: &Move, stacks: &mut [Vec<char>; 9]) {
    let mut buffer: Vec<char> = Vec::new();
    for _ in 0..m.num {
        let c = stacks[m.source].pop().expect("Could not pop from source");
        buffer.push(c);
    }
    for _ in 0..buffer.len() {
        let c = buffer.pop().expect("Could not pop from buffer");
        stacks[m.dest].push(c);
    }
}

fn main() {
    let mut stacks_part1 = initialize_stacks();
    let mut stacks_part2 = initialize_stacks();

    // read lines from the file one-by-one
    let file = File::open("data/day5.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let m = parse_move(&line);
        execute_move_part1(&m, &mut stacks_part1);
        execute_move_part2(&m, &mut stacks_part2);
    }

    println!("Part 1: {}", stack_tops(&stacks_part1));
    println!("Part 2: {}", stack_tops(&stacks_part2));
}
