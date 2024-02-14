use std::collections::{HashSet, VecDeque};

fn main() {
    // Use command line arguments to specify the input filename.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: ./main <input-file>\nNo input file provided. Exiting.");
    }

    // Next, read the contents of the input file into a string for easier processing.
    let input = std::fs::read_to_string(&args[1]).expect("Error opening file");

    // Use a double-ended queue as a ringbuffer to keep track of the characters.
    let mut deq = VecDeque::from([' '; 4]);

    // Iterate through all characters, together with their respective indices.
    for (i, char) in input.chars().enumerate() {
        // Remove the first character from the deque.
        deq.pop_front();
        // And add the next character to it.
        deq.push_back(char);
        // Collecting into a set, removing duplicates along the way.
        let hs: HashSet<char> = deq.iter().copied().collect();
        // Count the number of unique elements in the set.
        if hs.len() >= 4 && !hs.contains(&' ') {
            println!("Start of packet: {}", i + 1);
            break;
        }
    }
}
