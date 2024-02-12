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
    let input = input.lines();

    // --- TASK BEGIN ---
    let mut count = 0;

    for line in input {
        // Turn "1-2,3-6" into ["1-2", "3-6"].
        let ranges = line.split(',').collect::<Vec<_>>();
        // Turn ["1-2", "3-6"] into [["1", "2"], ["3", "6"]]
        let ranges = ranges
            .iter()
            .map(|x| x.split('-').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // Turn [["1", "2"], ["3", "6"]] into [1..=2, 3..=6]
        let ranges = ranges
            .iter()
            .map(|x| (x[0].parse::<i32>().unwrap())..=(x[1].parse::<i32>().unwrap()))
            .collect::<Vec<_>>();
        // Now turn the ranges into HashSets containing the respective integers. Essentially:
        // Turn [1..=2, 3..=6] into [{1,2}, {3,4,5,6}]
        let sets = ranges
            .into_iter()
            .map(|x| x.collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        // Check if the two sets are disjoint.
        // If there's at least one overlapping element, add it to the count.
        if !sets[0].is_disjoint(&sets[1]) {
            count += 1;
        }
    }

    println!("Count: {}", count);
}
