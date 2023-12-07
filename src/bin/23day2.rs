use regex::Regex;

// File containing puzzle data
const DATA: &str = "data/23day2.txt";

#[derive(Debug)]
struct Game {
    id: i32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

/// Get all `Game`s from the input file
fn get_games() -> Vec<Game> {
    let data = std::fs::read_to_string(DATA).unwrap();
    let games = data.split("\n").map(|line| parse_game(line)).collect();
    return games;
}

/// Parse the `Game` from a line of the input file
fn parse_game(line: &str) -> Game {
    // every line has the form "Game <number>: <cubes>; <cubes>; ..."
    let mut iter = line.split(":");
    let game_id = parse_game_id(iter.next().unwrap());
    let cube_sets = parse_cube_sets(iter.next().unwrap());
    return Game {
        id: game_id,
        cube_sets: cube_sets,
    };
}

fn parse_game_id(s: &str) -> i32 {
    // `s` has the form "Game <number>"
    let re = Regex::new(r"Game ([0-9]+)").unwrap();
    let game_id: i32 = re.captures(s).unwrap()[1].parse().unwrap();
    return game_id;
}

fn parse_cube_sets(s: &str) -> Vec<CubeSet> {
    // `s` has the form "<cubes>; <cubes>; ..."
    let mut cube_sets: Vec<CubeSet> = Vec::new();

    let red_re = Regex::new(r"([0-9]+) red").unwrap();
    let green_re = Regex::new(r"([0-9]+) green").unwrap();
    let blue_re = Regex::new(r"([0-9]+) blue").unwrap();

    for part in s.split(";") {
        let red: i32 = match red_re.captures(part) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0,
        };
        let green: i32 = match green_re.captures(part) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0,
        };
        let blue: i32 = match blue_re.captures(part) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0,
        };

        let cube_set = CubeSet { red, green, blue };
        cube_sets.push(cube_set);
    }
    return cube_sets;
}

fn compute_power(game: &Game) -> i32 {
    // first, find max number of each color
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for cube_set in &game.cube_sets {
        if cube_set.red > max_red {
            max_red = cube_set.red;
        }
        if cube_set.green > max_green {
            max_green = cube_set.green;
        }
        if cube_set.blue > max_blue {
            max_blue = cube_set.blue;
        }
    }

    // compute power
    let power = max_red * max_green * max_blue;

    return power;
}

fn main() {
    let games = get_games();

    // Part 1
    {
        // number of cubes in the bag
        let num_red = 12;
        let num_green = 13;
        let num_blue = 14;

        let mut sum_id = 0;

        'outer: for game in &games {
            for cube_set in &game.cube_sets {
                // check if cube set is possible
                if cube_set.red > num_red || cube_set.green > num_green || cube_set.blue > num_blue {
                    continue 'outer;
                }
            }
            // if possible, sum game id
            sum_id += game.id;
        }

        println!("Part 1: {}", sum_id); // Expected output: 3099
    }

    // Part 2
    {
        let sum_powers: i32 = games.iter().map(|game| compute_power(game)).sum();
        println!("Part 2: {}", sum_powers); // Expected output: 72970
    }
}
