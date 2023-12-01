// File containing puzle data
const DATA: &str = "data/23day1.txt";

const WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Given a line of text, get the (index, value) pairs for the first and last
/// digits.
fn get_first_last_digits(s: &str) -> ((usize, u32), (usize, u32)) {
    let mut index_digit_iter = s.chars().enumerate().filter(|(_, c)| c.is_digit(10)).peekable();
    let first = index_digit_iter.peek().unwrap().clone();
    let last = index_digit_iter.last().unwrap();
    // convert `char` to `u32`
    let first = (first.0, first.1.to_digit(10).unwrap());
    let last = (last.0, last.1.to_digit(10).unwrap());
    return (first, last);
}

/// Given a line of text, get the (index, value) pairs for all words "one",
/// "two", ..., "nine" found in the line.
fn get_all_words(s: &str) -> Vec<(usize, u32)> {
    let mut index_value_pairs: Vec<(usize, u32)> = Vec::new();
    for (value, word) in WORDS.iter().enumerate() {
        for (index, _) in s.match_indices(word) {
            index_value_pairs.push((index, u32::try_from(value).unwrap()));
        }
    }
    return index_value_pairs;
}

/// Parse the calibration value from a line of text. If `search_words` is true,
/// then searches for words "one", "two", ..., "nine" in the line.
fn parse_calibration_value(s: &str, search_words: bool) -> u32 {
    let (mut first, mut last) = get_first_last_digits(s);
    if search_words {
        for (index, value) in get_all_words(s) {
            if index < first.0 {
                first = (index, value);
            }
            if index > last.0 {
                last = (index, value);
            }
        }
    }
    return first.1 * 10 + last.1;
}

fn main() {
    // get document lines
    let document = std::fs::read_to_string(DATA).unwrap();
    let lines: Vec<&str> = document.split("\n").collect();

    // get calibration values for Part 1
    {
        let sum: u32 = lines.iter().map(|line| parse_calibration_value(line, false)).sum();
        println!("Part 1: {}", sum);
    }

    // get calibration values for Part 2
    {
        let sum: u32 = lines.iter().map(|line| parse_calibration_value(line, true)).sum();
        println!("Part 2: {}", sum);
    }
}
