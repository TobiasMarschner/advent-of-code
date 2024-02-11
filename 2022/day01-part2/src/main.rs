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
    let mut max_cals = [0u32, 0, 0];
    let mut cals = 0u32;

    // Iterate line-by-line.
    for line in input {
        match line.parse::<u32>() {
            Ok(num) => cals += num,
            Err(_) => {
                update_cals(cals, &mut max_cals);
                cals = 0;
            }
        }
    }
    // Don't forget to check the very last block.
    update_cals(cals, &mut max_cals);

    println!("Maximum Calories");
    for (i, mc) in max_cals.into_iter().enumerate() {
        println!("  No. {} : {}", i + 1, mc);
    }

    println!("Total Calories: {}", max_cals.iter().sum::<u32>())
}

fn update_cals(cals: u32, max_cals: &mut [u32; 3]) {
    if cals > max_cals[0] {
        max_cals[2] = max_cals[1];
        max_cals[1] = max_cals[0];
        max_cals[0] = cals;
    } else if cals > max_cals[1] {
        max_cals[2] = max_cals[1];
        max_cals[1] = cals;
    } else if cals > max_cals[2] {
        max_cals[2] = cals;
    }
}
