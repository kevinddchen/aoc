use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn parse_opponent_shape(line: &String) -> Shape {
    match line.chars().nth(0) {
        Some('A') => Shape::Rock,
        Some('B') => Shape::Paper,
        Some('C') => Shape::Scissors,
        _ => panic!("Unexpected shape: {}", line),
    }
}

// NOTE: uncomment this function to solve part 1
// fn parse_player_shape(line: &String) -> Shape {
//     match line.chars().nth(2) {
//         Some('X') => Shape::Rock,
//         Some('Y') => Shape::Paper,
//         Some('Z') => Shape::Scissors,
//         _ => panic!("Unexpected shape: {}", line),
//     }
// }

fn parse_player_shape(line: &String) -> Shape {
    // player shape is based on the opponent's shape
    let opponent_shape = parse_opponent_shape(&line);

    match line.chars().nth(2) {
        // need to lose
        Some('X') => match opponent_shape {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        // need to tie
        Some('Y') => opponent_shape,
        // need to win
        Some('Z') => match opponent_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        _ => panic!("Unexpected shape: {}", line),
    }
}

fn is_tie(opponent_shape: &Shape, player_shape: &Shape) -> bool {
    opponent_shape == player_shape
}

fn is_player_winner(opponent_shape: &Shape, player_shape: &Shape) -> bool {
    match opponent_shape {
        Shape::Rock => player_shape == &Shape::Paper,
        Shape::Paper => player_shape == &Shape::Scissors,
        Shape::Scissors => player_shape == &Shape::Rock,
    }
}

fn shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn main() {
    let mut total_score = 0;

    // read lines from the file one-by-one
    let file = File::open("data/day2.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let opponent_shape = parse_opponent_shape(&line);
        let player_shape = parse_player_shape(&line);

        // compute score for win/lose/tie
        if is_tie(&opponent_shape, &player_shape) {
            total_score += 3;
        } else if is_player_winner(&opponent_shape, &player_shape) {
            total_score += 6;
        }

        // add shape score
        total_score += shape_score(&player_shape);
    }

    println!("Total score: {}", total_score);
}
