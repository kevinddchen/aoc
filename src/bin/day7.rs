use advent_2022::forest::{Forest, NodeId};
use std::fs::File as IOFile;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
struct File {
    name: String, // this is unused, but still nice to have
    size: i32,
}

struct Dir {
    name: String,
    files: Vec<File>,
    size: i32, // this must be updated when files are added
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            name,
            files: Vec::new(),
            size: 0,
        }
    }
}

/// Create a file in the directory `dir`.
fn create_file(fs: &mut Forest<Dir>, dir: NodeId, file_name: String, file_size: i32) {
    let file = File {
        name: file_name,
        size: file_size,
    };
    fs.get_data_mut(dir).files.push(file);

    // Add the file size to the directory sizes
    let mut current_dir = dir;
    fs.get_data_mut(current_dir).size += file_size;
    while let Some(parent) = fs.get_parent(current_dir) {
        fs.get_data_mut(parent).size += file_size;
        current_dir = parent;
    }
}

/// Create a subdirectory in the directory `dir`.
fn create_subdir(fs: &mut Forest<Dir>, dir: NodeId, subdir_name: String) {
    fs.add_child(dir, Dir::new(subdir_name));
}

/// Return the subdirectory of `dir` with the given name.
fn get_subdir(fs: &mut Forest<Dir>, dir: NodeId, subdir_name: String) -> NodeId {
    for child in fs.get_children(dir) {
        if fs.get_data(child).name == subdir_name {
            return child;
        }
    }
    panic!("Could not find subdir: {}", subdir_name);
}

enum LineInstruction {
    CDROOT,
    CDIN(String),
    CDOUT,
    LS,
    DIR(String),
    FILE(i32, String),
}

fn parse_line_instruction(line: &str) -> LineInstruction {
    let words: Vec<&str> = line.split(' ').collect();
    match words[0] {
        "$" => match words[1] {
            "cd" => match words[2] {
                "/" => LineInstruction::CDROOT,
                ".." => LineInstruction::CDOUT,
                _ => LineInstruction::CDIN(words[2].to_string()),
            },
            "ls" => LineInstruction::LS,
            _ => panic!("Unknown instruction: {}", words[1]),
        },
        "dir" => LineInstruction::DIR(words[1].to_string()),
        _ => LineInstruction::FILE(words[0].parse::<i32>().expect("Could not parse file size"), words[1].to_string()),
    }
}

/// Execute a line instruction.
///
/// # Arguments
///
/// * `fs` - The file system.
/// * `dir` - The current directory.
/// * `instruction` - The instruction to execute.
fn execute_line_instruction(fs: &mut Forest<Dir>, dir: NodeId, instruction: LineInstruction) -> NodeId {
    match instruction {
        LineInstruction::CDROOT => dir, // HACK: do nothing, since this is only called in the beginning
        LineInstruction::CDIN(name) => get_subdir(fs, dir, name),
        LineInstruction::CDOUT => fs.get_parent(dir).expect("Could not get parent"),
        LineInstruction::LS => dir, // do nothing, since the subsequent lines list dirs/files
        LineInstruction::DIR(name) => {
            create_subdir(fs, dir, name);
            dir
        }
        LineInstruction::FILE(size, name) => {
            create_file(fs, dir, name, size);
            dir
        }
    }
}

/// Create the file system described in the input file. Returns the tree and the root node.
fn create_fs() -> (Forest<Dir>, NodeId) {
    let mut fs: Forest<Dir> = Forest::new();
    let root = fs.new_node(Dir::new(String::new()));
    let mut dir = root;

    // read lines from the file one-by-one
    let file = IOFile::open("data/day7.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let instruc = parse_line_instruction(&line);
        dir = execute_line_instruction(&mut fs, dir, instruc);
    }

    return (fs, root);
}

// --- Part 1 -----------------------------------------------------------------

/// Traverse the tree using DPS. Returns the sum of all subdirectories of `dir`
/// whose sizes are less than `size_limit`.
fn compute_sum_recursive(fs: &Forest<Dir>, dir: NodeId, size_limit: i32) -> i32 {
    let mut output = 0;

    // add recursive sum over children
    for child in fs.get_children(dir) {
        output += compute_sum_recursive(fs, child, size_limit);
    }

    // add size of current directory if it is less than the limit
    let dir_size = fs.get_data(dir).size;
    if dir_size <= size_limit {
        output += dir_size;
    }

    return output;
}

// --- Part 2 -----------------------------------------------------------------

/// Traverses the tree using DPS. Returns the size of the smallest subdirectory
/// of `dir` whose size is greater or equal to `target_size` (if any).
fn find_size_recursive(fs: &Forest<Dir>, dir: NodeId, target_size: i32) -> Option<i32> {
    let mut output: Option<i32> = None;

    // for each child,
    for child in fs.get_children(dir) {
        if fs.get_data(child).size < target_size {
            continue;
        }
        if let Some(size) = find_size_recursive(fs, child, target_size) {
            if output == None || size < output.unwrap() {
                output = Some(size);
            }
        }
    }

    // the current directory could be a candidate
    let dir_size = fs.get_data(dir).size;
    if output == None && dir_size >= target_size {
        output = Some(dir_size);
    }

    return output;
}

// ----------------------------------------------------------------------------

fn main() {
    let (fs, root) = create_fs();

    let sum_sizes = compute_sum_recursive(&fs, root, 100_000);
    println!("Part 1: {sum_sizes}");

    let total_size = fs.get_data(root).size; // total size is 41,072,511
    let target_size = total_size - 40_000_000; // need to find a folder whose size is >= 1,072,511

    let min_size = find_size_recursive(&fs, root, target_size).expect("Could not find target size");
    println!("Part 2: {min_size}");
}
