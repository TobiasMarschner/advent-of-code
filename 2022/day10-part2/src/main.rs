struct Crt {
    screen: [bool; 40 * 6],
}

impl Crt {
    fn new() -> Crt {
        Crt {
            screen: [false; 40 * 6],
        }
    }

    fn process_cycle(&mut self, cycle: usize, x: i32) {
        // Split cycle into corresponding line and column.
        let col = (cycle as i32) % 40;

        // Set the current pixel if the current column and the
        // sprite painted by the current x value overlap.
        self.screen[cycle] = (x - 1) == col || x == col || (x + 1) == col;
    }

    fn print(&self) {
        for (i, x) in self.screen.iter().enumerate() {
            // Print `##` or `. ` depending on bool value.
            print!("{}", if *x { "##" } else { ". " });
            // Print a newline every 40 characters.
            if (i + 1) % 40 == 0 {
                println!();
            }
        }
    }
}

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

    // Reserve the CRT and ...
    let mut crt = Crt::new();

    // ... iterate over all cycles to compute what would be shown on the screen.
    for (i, x) in x_over_time.iter().enumerate() {
        crt.process_cycle(i, *x);
    }

    // Finally, print said screen.
    crt.print();
}
