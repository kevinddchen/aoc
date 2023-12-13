/// File containing puzzle data
const DATA: &'static str = "data/22day7.txt";

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String, // unused
    size: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Dir {
    name: String, // unused
    size: i32,    // this must be updated when files and subdirs are added
    files: Vec<File>,
    subdirs: Vec<Dir>,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            name,
            size: 0,
            files: Vec::new(),
            subdirs: Vec::new(),
        }
    }
}

/// Encodes an executed bash instruction or an output line of an `ls` call.
#[derive(Debug)]
enum Instruction {
    CDROOT,       // cd to the root directory
    CDIN(String), // cd to the given directory
    CDOUT,        // cd to the parent directory
    LS,           // list the contents of the current directory
    DIR(String),  // name of a subdirectory, from `ls`
    FILE(File),   // file in the current directory, from `ls`
}

/// Parses an `Instruction` from a line of the input file.
fn parse_instruction(line: &str) -> Instruction {
    let words: Vec<&str> = line.split(' ').collect();
    match words[0] {
        "$" => match words[1] {
            "cd" => match words[2] {
                "/" => Instruction::CDROOT,
                ".." => Instruction::CDOUT,
                _ => Instruction::CDIN(words[2].to_string()),
            },
            "ls" => Instruction::LS,
            _ => panic!("Unknown instruction: {}", words[1]),
        },
        "dir" => Instruction::DIR(words[1].to_string()),
        _ => Instruction::FILE(File {
            size: words[0].parse().unwrap(),
            name: words[1].to_string(),
        }),
    }
}

/// Execute a line instruction on the current state of the file system.
fn execute_instruction(dir_stack: &mut Vec<Dir>, instruction: Instruction) {
    match instruction {
        Instruction::CDIN(name) => {
            // cd into the given directory
            dir_stack.push(Dir::new(name));
        }
        Instruction::CDOUT => {
            // on the way out, add the current directory as a subdir of the parent
            let dir = dir_stack.pop().unwrap();
            let parent = dir_stack.last_mut().unwrap();
            parent.size += dir.size;
            parent.subdirs.push(dir);
        }
        Instruction::FILE(file) => {
            // add the file to the current directory
            let dir = dir_stack.last_mut().unwrap();
            dir.size += file.size;
            dir.files.push(file);
        }
        _ => {} // do nothing for all other instructions
    };
}

/// Create the file system.
fn create_fs() -> Dir {
    let data = std::fs::read_to_string(DATA).unwrap();

    // construct the file system by executing the instructions in the input file
    let mut dir_stack = vec![Dir::new(String::from("/"))];
    for line in data.split('\n') {
        let instruction = parse_instruction(line);
        execute_instruction(&mut dir_stack, instruction);
    }

    // if the directory stack is non-empty, we need to cd out to the "/" root
    // directory and add subdirectories along the way
    while dir_stack.len() > 1 {
        execute_instruction(&mut dir_stack, Instruction::CDOUT);
    }
    return dir_stack.pop().unwrap();
}

fn main() {
    // initialize the file system
    let fs = create_fs();

    // Part 1
    {
        // recursively search for directories with size less than or equal to
        // 100,000 and sum their sizes
        fn sum_dir_sizes(dir: &Dir, limit: i32) -> i32 {
            let mut total = 0;
            for subdir in &dir.subdirs {
                total += sum_dir_sizes(subdir, limit);
            }
            if dir.size < limit {
                total += dir.size;
            }
            return total;
        }

        let sum = sum_dir_sizes(&fs, 100_000);
        println!("Part 1: {}", sum); // Expected output: 1989474
    }

    // Part 2
    {
        let total_size = fs.size; // total size is 41,072,511
        let threshold_size = total_size - 40_000_000; // need to find a folder whose size is >= 1,072,511

        // recursively search for the smallest directory with size greater or
        // equal to `threshold_size`
        fn find_dir_smallest(dir: &Dir, threshold: i32) -> Option<i32> {
            if dir.size < threshold {
                return None;
            }

            let mut smallest = Some(dir.size);
            for subdir in &dir.subdirs {
                if let Some(size) = find_dir_smallest(subdir, threshold) {
                    if size < smallest.unwrap() {
                        smallest = Some(size);
                    }
                }
            }
            return smallest;
        }

        let smallest = find_dir_smallest(&fs, threshold_size).unwrap();
        println!("Part 2: {}", smallest); // Expected output: 1111607
    }
}
