use advent_2022::vec::Vec2d;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Motion {
    Up,
    Right,
    Down,
    Left,
}

struct Rope {
    head: Vec2d,
    tail: Vec2d,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Vec2d { x: 0, y: 0 },
            tail: Vec2d { x: 0, y: 0 },
        }
    }

    /// Returns true if the points are touching
    fn is_touching(a: &Vec2d, b: &Vec2d) -> bool {
        (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
    }

    /// Move the rope according to the given motion.
    fn move_rope(&mut self, motion: &Motion){
        // find new head position
        let dir_vec = match motion {
            Motion::Up => Vec2d { x: 0, y: 1 },
            Motion::Right => Vec2d { x: 1, y: 0 },
            Motion::Down => Vec2d { x: 0, y: -1 },
            Motion::Left => Vec2d { x: -1, y: 0 },
        };
        let new_head = self.head + dir_vec;

        if !Rope::is_touching(&new_head, &self.tail) {
            // New head is not touching, so tail moves to old head position
            self.tail = self.head;
        }
        self.head = new_head;
    }

    /// Returns list of tail positions after moving the rope according to the
    /// given motions.
    fn get_tail_movement(&mut self, motions: &Vec<Motion>) -> Vec<Vec2d> {
        let mut tail_positions: Vec<Vec2d> = Vec::new();

        for motion in motions {
            self.move_rope(motion);
            tail_positions.push(self.tail);
        }

        return tail_positions;
    }
}

/// Read all motions from file.
fn get_motions() -> Vec<Motion> {
    let mut motions: Vec<Motion> = Vec::new();

    // read lines from the file one-by-one
    let file = File::open("data/day9.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let chars: Vec<&str> = line.split(' ').collect();
        let steps: i32 = chars[1].parse().expect("Could not parse steps");
        let motion = match chars[0] {
            "U" => Motion::Up,
            "R" => Motion::Right,
            "D" => Motion::Down,
            "L" => Motion::Left,
            _ => panic!("Invalid direction"),
        };

        for _ in 0..steps {
            motions.push(motion);
        }
    }

    return motions;
}

fn count_unique_tail_positions(rope: &mut Rope, motions: &Vec<Motion>) -> i32 {
    let tail_positions = rope.get_tail_movement(motions);
    let mut unique_tail_positions: HashSet<Vec2d> = HashSet::new();

    let mut count = 0;
    for tail_pos in tail_positions {
        if unique_tail_positions.insert(tail_pos) {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let mut rope = Rope::new();
    let motions = get_motions();

    let count = count_unique_tail_positions(&mut rope, &motions);
    println!("Part 1: {}", count);
}
