// File containing puzzle data
const DATA: &str = "data/22day2.txt";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum EncodedShape {
    X,
    Y,
    Z,
}

/// Parse shape from character A, B, or C.
fn parse_shape(s: char) -> Shape {
    match s {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("Unexpected shape: {}", s),
    }
}

/// Parse encoded shape from character X, Y, or Z.
fn parse_encoded_shape(s: char) -> EncodedShape {
    match s {
        'X' => EncodedShape::X,
        'Y' => EncodedShape::Y,
        'Z' => EncodedShape::Z,
        _ => panic!("Unexpected encoded shape: {}", s),
    }
}

/// Returns the score of the given shape.
fn shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

/// Returns the shape that is stronger than the given shape.
fn stronger_shape(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

/// Returns the shape that is weaker than the given shape.
fn weaker_shape(shape: &Shape) -> Shape {
    stronger_shape(&stronger_shape(shape))
}

// Compute the player's score for the round.
fn score_round(opponent_shape: &Shape, player_shape: &Shape) -> i32 {
    let mut score = shape_score(player_shape);
    // if player beats opponent
    if stronger_shape(opponent_shape) == *player_shape {
        score += 6;
    // if player ties opponent
    } else if *opponent_shape == *player_shape {
        score += 3;
    }
    return score;
}

/// Get the opponent shape and player encoded shape for each round.
fn get_rounds() -> Vec<(Shape, EncodedShape)> {
    let mut rounds: Vec<(Shape, EncodedShape)> = Vec::new();

    // iterate over lines of the input file
    let data = std::fs::read_to_string(DATA).unwrap();

    // each line has the format "<A|B|C> <X|Y|Z>"
    for line in data.split("\n") {
        let shape = parse_shape(line.chars().nth(0).unwrap());
        let encoded_shape = parse_encoded_shape(line.chars().nth(2).unwrap());
        rounds.push((shape, encoded_shape));
    }

    return rounds;
}

fn decode_part1(encoded_shape: &EncodedShape) -> Shape {
    match encoded_shape {
        EncodedShape::X => Shape::Rock,
        EncodedShape::Y => Shape::Paper,
        EncodedShape::Z => Shape::Scissors,
    }
}

fn decode_part2(opponent_shape: &Shape, encoded_shape: &EncodedShape) -> Shape {
    match encoded_shape {
        EncodedShape::X => weaker_shape(opponent_shape),
        EncodedShape::Y => *opponent_shape,
        EncodedShape::Z => stronger_shape(opponent_shape),
    }
}

fn main() {
    let rounds = get_rounds();

    // Part 1
    {
        let mut sum_points = 0;
        for (opponent_shape, player_encoded_shape) in &rounds {
            let player_shape = decode_part1(player_encoded_shape);
            let points = score_round(opponent_shape, &player_shape);
            sum_points += points;
        }
        println!("Part 1: {}", sum_points); // Expected output: 15572
    }

    // Part 2
    {
        let mut sum_points = 0;
        for (opponent_shape, player_encoded_shape) in &rounds {
            let player_shape = decode_part2(opponent_shape, player_encoded_shape);
            let points = score_round(opponent_shape, &player_shape);
            sum_points += points;
        }
        println!("Part 2: {}", sum_points); // Expected output: 16098
    }
}
