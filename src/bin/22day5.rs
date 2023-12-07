// File containing puzzle data
const DATA: &str = "data/22day5.txt";

#[derive(Debug)]
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

/// Parse the moves from the input file
fn get_moves() -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let data = std::fs::read_to_string(DATA).unwrap();
    for line in data.split("\n") {
        let parts: Vec<&str> = line.split(' ').collect();
        moves.push(Move {
            num: parts[1].parse().unwrap(),
            source: parts[3].parse::<usize>().unwrap() - 1,
            dest: parts[5].parse::<usize>().unwrap() - 1,
        });
    }

    return moves;
}

/// Concatenates the characters at the top of each stack into a string
fn stack_tops(stacks: &[Vec<char>; 9]) -> String {
    let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    return result;
}

/// Execute the move on the stacks. If `chunk` is false, crates are moved one
/// at a time, e.g. in Part 1. If `chunk` is true, crates are moved all
/// together, e.g. in Part 2.
fn execute_move(m: &Move, stacks: &mut [Vec<char>; 9], chunk: bool) {
    // buffer holds popped crates in reverse order
    let mut buffer: Vec<char> = Vec::new();
    for _ in 0..m.num {
        buffer.push(stacks[m.source].pop().unwrap());
    }

    let buffer_iter: Box<dyn Iterator<Item=_>> = if chunk {
        Box::new(buffer.iter().rev())
    } else {
        Box::new(buffer.iter())
    };
    stacks[m.dest].extend(buffer_iter);
}

fn main() {
    let moves = get_moves();

    // Part 1
    {
        let mut stacks = initialize_stacks();
        for m in moves.iter() {
            execute_move(m, &mut stacks, false);
        }
        println!("Part 1: {}", stack_tops(&stacks)); // Expected output: SHQWSRBDL
    }

    // Part 2
    {
        let mut stacks = initialize_stacks();
        for m in moves.iter() {
            execute_move(m, &mut stacks, true);
        }
        println!("Part 2: {}", stack_tops(&stacks)); // Expected output: CDTQZHBRS
    }
}
