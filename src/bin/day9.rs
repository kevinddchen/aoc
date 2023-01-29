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
    length: usize,
    knots: Vec<Vec2d>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut knots: Vec<Vec2d> = Vec::new();
        for _ in 0..length {
            knots.push(Vec2d { x: 0, y: 0 });
        }
        Self { length, knots }
    }

    /// Returns true if the points are touching
    fn is_touching(a: &Vec2d, b: &Vec2d) -> bool {
        (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
    }

    /// Returns the new position of a tail knot given the new position of the head knot
    fn move_towards(tail: &Vec2d, head: &Vec2d) -> Vec2d {
        if Self::is_touching(tail, head) {
            // tail and head are touching, so tail does not move
            return *tail;
        }
        // move x of tail towards head
        let mut x = tail.x;
        if tail.x < head.x {
            x += 1;
        } else if tail.x > head.x {
            x -= 1;
        }
        // move y of tail towards head
        let mut y = tail.y;
        if tail.y < head.y {
            y += 1;
        } else if tail.y > head.y {
            y -= 1;
        }
        return Vec2d { x, y };
    }

    /// Move the rope according to the given motion.
    fn move_rope(&mut self, motion: &Motion) {
        let mut new_knots: Vec<Vec2d> = Vec::new();

        // find new head position
        let dir_vec = match motion {
            Motion::Up => Vec2d { x: 0, y: 1 },
            Motion::Right => Vec2d { x: 1, y: 0 },
            Motion::Down => Vec2d { x: 0, y: -1 },
            Motion::Left => Vec2d { x: -1, y: 0 },
        };
        new_knots.push(self.knots[0] + dir_vec);

        // move all other knots
        for i in 1..self.length {
            new_knots.push(Self::move_towards(&self.knots[i], &new_knots[i - 1]));
        }

        self.knots = new_knots;
    }

    /// Returns list of tail positions after moving the rope according to the
    /// given motions.
    fn get_tail_movement(&mut self, motions: &Vec<Motion>) -> Vec<Vec2d> {
        let mut tail_positions: Vec<Vec2d> = Vec::new();

        for motion in motions {
            self.move_rope(motion);
            let tail = self.knots[self.length - 1];
            tail_positions.push(tail);
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
    let motions = get_motions();

    let mut rope = Rope::new(2);
    let count = count_unique_tail_positions(&mut rope, &motions);
    println!("Part 1: {}", count);

    let mut rope = Rope::new(10);
    let count = count_unique_tail_positions(&mut rope, &motions);
    println!("Part 2: {}", count);
}
