#[derive(Copy, Clone, Debug)]
struct MoveOperation {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    // Use command line arguments to specify the input filename.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!(
            "Usage: ./main <input-file> <number-of-lanes>\nNot enough arguments provided. Exiting."
        );
    }

    // Next, read the contents of the input file into a string for easier processing.
    let input = std::fs::read_to_string(&args[1]).expect("Error opening file");
    let lane_count = args[2].parse::<usize>().unwrap();
    // Line-by-line processing is easiest.
    let mut input = input.lines();

    // --- TASK BEGIN ---

    // First of all, parse the text input into our own data structures for easier solving.

    // Create the data structure representing the cargo hold.
    let mut cargo_hold: Vec<Vec<char>> = Vec::new();
    for _ in 0..lane_count {
        cargo_hold.push(Vec::new());
    }

    loop {
        // Split the input line into chunks, each possibly representing a box.
        let line = input.next().unwrap();

        // If we've reached the line indicating the stack numbers, we're done here.
        // Break out of the loop and continue parsing the move operations.
        if line.chars().nth(1).unwrap() == '1' {
            input.next();
            break;
        }

        // Iterate over all stacks in the cargo hold.
        for (i, stack) in cargo_hold.iter_mut().enumerate() {
            // Get the character for this particular stack.
            let c = line.chars().nth(i * 4 + 1).unwrap();
            if c != ' ' {
                stack.insert(0, c);
            }
        }
    }

    // Next, parse all of the move oprations.

    // Create the data structure holding all of the move operations.
    let mut move_operations: Vec<MoveOperation> = Vec::new();

    for line in input {
        // Turn "move x from y to z" into ["move", "x", "from", "y", "to", "z"]
        let words = line.split(' ').collect::<Vec<_>>();
        // Parse x, y and z and create a new MoveOperation with it.
        move_operations.push(MoveOperation {
            amount: words[1].parse::<usize>().unwrap(),
            from: words[3].parse::<usize>().unwrap() - 1,
            to: words[5].parse::<usize>().unwrap() - 1,
        });
    }

    // Now that we have all of the data, start executing.
    // Iterate over all move opertaions.
    for mop in &move_operations {
        // We move boxes as a whole stack.
        // Determine the size of the "from"-stack.
        let stack_size = cargo_hold[mop.from].len();
        // Cut off the amount required from the "from"-stack and move it to "cargo".
        let mut cargo = cargo_hold[mop.from].split_off(stack_size - mop.amount);
        // And then add those boxes to the destination stack.
        cargo_hold[mop.to].append(&mut cargo);
    }

    // Print the string with each stack's topmost cargo.
    print!("Solution: ");
    for stack in &cargo_hold {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
