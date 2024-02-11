use std::collections::HashSet;

fn main() {
    // Use command line arguments to specify the input filename.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: ./main <input-file>\nNo input file provided. Exiting.");
    }

    // Next, read the contents of the input file into a string for easier processing.
    let input = std::fs::read_to_string(&args[1]).expect("Error opening file");
    // Line-by-line processing is easiest.
    let mut input = input.lines();

    // --- TASK BEGIN ---
    let mut total = 0;
    while let Some(line) = input.next() {
        // Read the next three lines and turn them into HashSets.
        let first: HashSet<char> = line.chars().collect();
        let second: HashSet<char> = input.next().unwrap().chars().collect();
        let third: HashSet<char> = input.next().unwrap().chars().collect();
        // Use set intersection to determine the item that's common to all three sets.
        let item = *first.intersection(&second).copied().collect::<HashSet<char>>().intersection(&third).next().unwrap();
        // Calculate the priority.
        let priority = match item {
            item if item.is_ascii_lowercase() => (item as u32) - ('a' as u32) + 1,
            item if item.is_ascii_uppercase() => (item as u32) - ('A' as u32) + 27,
            _ => {panic!("Character out of range")}
        };
        // Accumulate the priorities.
        total += priority;
    }
    println!("Total priority: {total}");
}

