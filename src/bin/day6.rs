use std::collections::HashSet;
use std::fs;

/// Given a string slice, return true if all characters are unique
fn all_unique_chars(s: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for c in s.chars() {
        if !chars.insert(c) {
            return false;
        }
    }
    return true;
}

/// Given a string slice, returns index at which the first contiguous run of
/// `size` unique characters is found
fn find_unique_run(size: usize, s: &str) -> usize {
    for i in 0..s.len() - size {
        let slice = &s[i..i + size];
        if all_unique_chars(slice) {
            return i;
        }
    }
    panic!("Could not find contiguous run of {size} unique characters");
}

fn main() {
    const SOP_MARKER_SIZE: usize = 4; // start-of-packet marker size
    const SOM_MARKER_SIZE: usize = 14; // start-of-message marker size

    let datastream = fs::read_to_string("data/day6.txt").expect("Could not read file");

    let sop_marker_index = find_unique_run(SOP_MARKER_SIZE, &datastream);
    let packet_start = sop_marker_index + SOP_MARKER_SIZE;
    println!("Skipped characters for start-of-packet: {packet_start}");

    let som_marker_index = find_unique_run(SOM_MARKER_SIZE, &datastream[packet_start..]);
    let message_start = som_marker_index + SOM_MARKER_SIZE + packet_start;
    println!("Skipped characters for start-of-message: {message_start}");
}
