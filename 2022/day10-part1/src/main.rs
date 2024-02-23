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

    // Keep track of the current value of X for all the instructions.
    // We'll use one massive vector for that purpose.
    let mut x_over_time: Vec<i32> = Vec::with_capacity(512);

    // Keep track of the actual x as well.
    let mut x = 1;

    // Then, process line-by-line.
    for line in input {
        // Split by space.
        let line: Vec<_> = line.split(' ').collect();

        // Differentiate by instruction
        match line[0] {
            "noop" => {
                // Nothing changes.
                x_over_time.push(x);
            }
            "addx" => {
                // Addition is complete *after* two cycles.
                // So during those two cycles x has the old value still.
                x_over_time.push(x);
                x_over_time.push(x);
                // Of course, afterwards the value of x is updated.
                x += line[1].parse::<i32>().unwrap();
            }
            _ => (),
        }
    }

    // dbg!(&x_over_time);

    // Afterwards, compute our signal strength result.
    let mut signal_strength = 0;
    for (i, x) in x_over_time.iter().skip(19).step_by(40).enumerate() {
        let cycle: i32 = 20 + (i as i32) * 40;
        signal_strength += cycle * x;
    }

    println!("Signal strength: {}", signal_strength);
}
