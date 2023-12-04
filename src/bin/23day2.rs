use regex::Regex;

// File containing puzle data
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


/// Parse all `Game`s from the input file
fn parse_games() -> Vec<Game> {
    let file = std::fs::read_to_string(DATA).unwrap();
    let games = file.split("\n").map(|line| parse_game(line)).collect();
    return games;
}

/// Parse the `Game` from a line of the input file
fn parse_game(line: &str) -> Game {
    // every line has the form "Game <number>: <cubes>; <cubes>; ..."
    let parts: Vec<&str> = line.split(":").collect();
    if parts.len() != 2 {
        panic!("Invalid line: {}", line);
    }
    let game_id = parse_game_id(parts[0]);
    let cube_sets = parse_cube_sets(parts[1]);
    return Game {
        id: game_id,
        cube_sets: cube_sets,
    };
}

fn parse_game_id(s: &str) -> i32 {
    let re = Regex::new(r"Game ([0-9]+)").unwrap();
    let game_id: i32 = re.captures(s).unwrap()[1].parse().unwrap();
    return game_id;
}

fn parse_cube_sets(s: &str) -> Vec<CubeSet> {
    let mut cube_sets: Vec<CubeSet> = Vec::new();

    let red_re = Regex::new(r"([0-9]+) red").unwrap();
    let green_re = Regex::new(r"([0-9]+) green").unwrap();
    let blue_re = Regex::new(r"([0-9]+) blue").unwrap();

    for part in s.split(";") {
        let reds: i32 = match red_re.captures(part) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0,
        };
        let greens: i32 = match green_re.captures(part) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0,
        };
        let blues: i32 = match blue_re.captures(part) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0,
        };

        let cube_set = CubeSet {
            red: reds,
            green: greens,
            blue: blues,
        };
        cube_sets.push(cube_set);
    }
    return cube_sets;
}

fn main() {
    let games = parse_games();

    // Part 1
    {
        // number of cubes in the bag
        let num_red = 12;
        let num_green = 13;
        let num_blue = 14;

        let mut id_sum = 0;

        'outer: for game in games {
            for cube_set in game.cube_sets {
                // check if cube set is possible
                if cube_set.red > num_red || cube_set.green > num_green || cube_set.blue > num_blue {
                    continue 'outer;
                }
            }
            id_sum += game.id;
        }

        println!("Part 1: {}", id_sum);
    }
    
}
