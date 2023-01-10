use std::fs;
use std::ops::AddAssign;

/// Parse heights from the input.
fn get_heights() -> Vec<Vec<i32>> {
    let raw = fs::read_to_string("data/day8.txt").expect("Could not read file");

    let mut height: Vec<Vec<i32>> = Vec::new();
    for raw_row in raw.split('\n') {
        let mut row: Vec<i32> = Vec::new();
        for c in raw_row.chars() {
            row.push(c.to_digit(10).expect("Could not parse digit") as i32);
        }
        if !row.is_empty() {
            height.push(row);
        }
    }

    return height;
}

#[derive(Copy, Clone)]
struct Vec2d {
    x: i32,
    y: i32,
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, other: Vec2d) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn initialize_visibilities(size: usize) -> Vec<Vec<bool>> {
    let mut visible: Vec<Vec<bool>> = Vec::new();
    for _ in 0..size {
        visible.push(vec![false; size]);
    }
    return visible;
}

/// `start` is a point on the edge of the map, and `dir` is a vector pointing inwards.
/// March along the grid in the direction of `dir`, marking all points as visible
/// if they are higher than the highest point seen so far.
fn mark_visibilities(visible: &mut Vec<Vec<bool>>, height: &Vec<Vec<i32>>, start: Vec2d, dir: Vec2d) {
    let size = height.len();

    let mut highest_so_far = -1;
    let mut pos = start;
    for _ in 0..size {
        let curr_height = height[pos.x as usize][pos.y as usize];
        if curr_height > highest_so_far {
            highest_so_far = curr_height;
            visible[pos.x as usize][pos.y as usize] = true;
        }
        pos += dir;
    }
}

#[rustfmt::skip]
fn get_visibilities(height: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let size = height.len();

    let mut visible = initialize_visibilities(size);

    for i in 0..size {
        let i: i32 = i as i32;
        let size: i32 = size as i32;
        // Start on bottom row
        mark_visibilities(&mut visible, height, Vec2d { x: i, y: 0 }, Vec2d { x: 0, y: 1 });
        // Start on top row
        mark_visibilities(&mut visible, height, Vec2d { x: i, y: size - 1 }, Vec2d { x: 0, y: -1 });
        // Start on left column
        mark_visibilities(&mut visible, height, Vec2d { x: 0, y: i }, Vec2d { x: 1, y: 0 });
        // Start on right column
        mark_visibilities(&mut visible, height, Vec2d { x: size - 1, y: i }, Vec2d { x: -1, y: 0 });
    }

    return visible;
}

fn count_visibilities(visible: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    for row in visible {
        for visible in row {
            if *visible {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let height = get_heights();
    let visible = get_visibilities(&height);

    let count = count_visibilities(&visible);
    println!("Part 1: {count}");
}
