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

    // Keep track of the largest calorie-count and the "current" calorie-count.
    let mut max_cals = 0u32;
    let mut cals = 0u32;

    // Iterate line-by-line.
    for line in input {
        // println!("Line is: {line}");
        match line.parse::<u32>() {
            Ok(num) => cals += num,
            Err(_) => {
                // println!("{cals}");
                max_cals = std::cmp::max(max_cals, cals);
                cals = 0;
            }
        }
    }
    // println!("{cals}");
    // Don't forget to check the very last block.
    max_cals = std::cmp::max(max_cals, cals);

    println!("Maximum calories: {max_cals}");
}
