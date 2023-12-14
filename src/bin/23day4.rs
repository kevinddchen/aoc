use std::collections::HashSet;

/// File containing puzzle data
const DATA: &'static str = "data/23day4.txt";

/// Given a string of whitespace-separated numbers, e.g. "45 12  3", returns a
/// vector of those numbers.
fn parse_whitespace_separated_numbers(s: &str) -> Vec<i32> {
    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/// Return list of cards containing (winning numbers, my numbers) pairs.
fn get_cards() -> Vec<(Vec<i32>, Vec<i32>)> {
    let data = std::fs::read_to_string(DATA).unwrap();
    let mut cards: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
    for line in data.split('\n') {
        // line is of the form: "Card #: # # # ... | # # # ..."
        let line_numbers = line.split(':').last().unwrap();
        // line_numbers is of the form: "# # # ... | # # # ..."
        let mut iter = line_numbers.split('|');
        let winning = parse_whitespace_separated_numbers(iter.next().unwrap());
        let my = parse_whitespace_separated_numbers(iter.next().unwrap());
        cards.push((winning, my));
    }
    return cards;
}

fn count_matches(winning: &Vec<i32>, my: &Vec<i32>) -> usize {
    // put winning numbers to a set
    let mut winning_set = HashSet::new();
    for &num in winning.iter() {
        winning_set.insert(num);
    }
    // count number of matches
    let matches = my.iter().filter(|&num| winning_set.contains(&num)).count();
    return matches;
}

fn compute_score(matches: usize) -> i32 {
    if matches == 0 {
        return 0;
    } else {
        return i32::pow(2, (matches - 1) as u32);
    }
}

fn main() {
    let cards = get_cards();

    // Part 1
    {
        let total_score: i32 = cards
            .iter()
            .map(|(w, o)| count_matches(w, o))
            .map(|m| compute_score(m))
            .sum();
        println!("Part 1: {}", total_score); // Expected output: 21088
    }

    // Part 2
    {
        // start with vector of number of copies of each card
        let mut copies: Vec<i32> = vec![1; cards.len()];

        for (card_idx, card) in cards.iter().enumerate() {
            let matches = count_matches(&card.0, &card.1);

            let num_copies = copies[card_idx];
            for i in 0..matches {
                let new_card_idx = card_idx + i + 1;
                copies[new_card_idx] += num_copies;
            }
        }

        let total_num_cards: i32 = copies.iter().sum();
        println!("Part 2: {}", total_num_cards); // Expected output: 6874754
    }
}
