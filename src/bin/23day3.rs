use regex::Regex;
use std::collections::HashMap;

/// File containing puzzle data
const DATA: &'static str = "data/23day3.txt";

/// A number in the engine, labelled by its coordinates and value. Note that a
/// "number" may not necessary be a "part".
#[derive(Debug)]
struct Number {
    row: usize,
    col: usize,
    length: usize,
    value: i32,
}

/// A symbol in the schematic, labelled by its coordinates and value.
#[derive(Debug)]
struct Symbol {
    row: usize,
    col: usize,
    value: char,
}

/// Read schematic from the text file
fn get_schematic() -> Vec<String> {
    let data = std::fs::read_to_string(DATA).unwrap();
    let lines: Vec<String> = data.split("\n").map(|s| s.to_string()).collect();
    return lines;
}

/// Given a schematic, returns a list of all numbers in the schematic
fn parse_numbers(schematic: &Vec<String>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    let re = Regex::new(r"([0-9]+)").unwrap();
    for (row, line) in schematic.iter().enumerate() {
        for cap in re.captures_iter(line) {
            let mat = cap.get(1).unwrap();
            let number = Number {
                row: row,
                col: mat.start(),
                length: mat.end() - mat.start(),
                value: cap[1].parse().unwrap(),
            };
            numbers.push(number);
        }
    }
    return numbers;
}

/// Given a number in a schematic, check if it is a part. If it is, returns
/// the adjacent symbol and its coordinates. Otherwise, returns None.
fn is_part(number: &Number, schematic: &Vec<String>) -> Option<Symbol> {
    let num_rows = isize::try_from(schematic.len()).unwrap();
    let num_cols = isize::try_from(schematic[0].len()).unwrap();

    // Returns the symbol at the coordinate, if any. The given coordinate can
    // be out of bounds.
    let get_symbol = |row: isize, col: isize| -> Option<Symbol> {
        if row < 0 || row >= num_rows || col < 0 || col >= num_cols {
            return None;
        }
        let row = usize::try_from(row).unwrap();
        let col = usize::try_from(col).unwrap();
        match schematic[row].chars().nth(col) {
            Some('.') => None,
            Some(value) => Some(Symbol { row, col, value }),
            _ => None,
        }
    };

    let row = isize::try_from(number.row).unwrap();
    let col = isize::try_from(number.col).unwrap();
    let length = isize::try_from(number.length).unwrap();

    // check directly left and right
    if let Some(s) = get_symbol(row, col - 1) {
        return Some(s);
    }
    if let Some(s) = get_symbol(row, col + length) {
        return Some(s);
    }
    // check rows above and below
    for x in (col - 1)..(col + length + 1) {
        if let Some(s) = get_symbol(row - 1, x) {
            return Some(s);
        }
        if let Some(s) = get_symbol(row + 1, x) {
            return Some(s);
        }
    }
    return None;
}

fn main() {
    let schematic = get_schematic();
    let numbers = parse_numbers(&schematic);

    // Part 1
    {
        let mut part_number_sum = 0;
        for number in &numbers {
            // the number is a part if it is adjacent to a symbol
            if let Some(_) = is_part(number, &schematic) {
                part_number_sum += number.value;
            }
        }
        println!("Part 1: {}", part_number_sum); // Expected output: 538046
    }

    // Part 2
    {
        // we collect a hash map of all encountered gear symbols. the key is
        // (row, col) and the value is a list of part numbers adjacent to the
        // gear symbol.
        let mut gear_to_numbers: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

        for number in &numbers {
            if let Some(symbol) = is_part(number, &schematic) {
                // `number` is a part
                if symbol.value == '*' {
                    gear_to_numbers
                        .entry((symbol.row, symbol.col))
                        .or_insert(Vec::new())
                        .push(number.value);
                }
            }
        }

        // now we iterate over all gears and filter for all those with exactly two adjacent parts
        let sum_gear_ratios: i32 = gear_to_numbers
            .values()
            .filter(|part_numbers| part_numbers.len() == 2)
            .map(|part_numbers| part_numbers.iter().product::<i32>())
            .sum();
        println!("Part 2: {}", sum_gear_ratios); // Expected output: 81709807
    }
}
