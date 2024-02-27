use std::collections::VecDeque;

// Represent the different operations to perform on the worry level.
#[derive(Copy, Clone, Debug)]
enum MonkeyOperation {
    Multiply(i64),
    Add(i64),
    Square,
}

// Represent all the data for an individual monkey.
#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op: MonkeyOperation,
    test_divisor: i64,
    true_dest: usize,
    false_dest: usize,
    inspect_count: i64,
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
    let mut input = input.lines();

    // --- TASK BEGIN ---

    // Parse and collect all of the data, first and foremost.
    let mut monkeys: Vec<Monkey> = Vec::new();

    // Iterate line-by-line.
    loop {
        // We don't care about the "Monkey 0" line.
        if input.next().is_none() {
            // End of the file? Then we're done.
            break;
        }

        // Parse the starting items.
        let (_, items) = input.next().unwrap().split_at(18);
        // Parse the actual numbers and collect them into an integer vector.
        let items: VecDeque<_> = items
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        // Parse the operation.
        let line: Vec<_> = input.next().unwrap().split_whitespace().collect();
        let op = match (line[4], line[5]) {
            ("*", "old") => MonkeyOperation::Square,
            ("*", x) => MonkeyOperation::Multiply(x.parse().unwrap()),
            ("+", x) => MonkeyOperation::Add(x.parse().unwrap()),
            (_, _) => {
                panic!("Could not parse operation");
            }
        };

        // Parse the number by which to divide.
        let test_divisor = input.next().unwrap().split_at(21).1.parse::<i64>().unwrap();

        // Parse the monkey destinations in the true and false case.
        let true_dest = input
            .next()
            .unwrap()
            .split_at(29)
            .1
            .parse::<usize>()
            .unwrap();
        let false_dest = input
            .next()
            .unwrap()
            .split_at(30)
            .1
            .parse::<usize>()
            .unwrap();

        // Skip the whitespace line.
        input.next();

        // Finally, actually construct the monkey out of all this and add it to the list.
        monkeys.push(Monkey {
            items,
            op,
            test_divisor,
            true_dest,
            false_dest,
            inspect_count: 0,
        })
    }

    // Determine the cooloff number.
    // All divisors multiplied with each other constitute the biggest safe number
    // that we can "cut off" off the worry level without affecting any of the calculations.
    let cooloff_number = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .reduce(|acc, e| acc * e)
        .unwrap();

    // Now that we have all the monkeys, start simulating.
    for _ in 0..10000 {
        // Go through all monkeys one-by-one.
        for m in 0..monkeys.len() {
            // Go through the queue of items, starting with the front.
            while let Some(item) = monkeys[m].items.pop_front() {
                // First, apply the monkey's operation.
                let newval = match monkeys[m].op {
                    // MonkeyOperation::Square => item * item,
                    MonkeyOperation::Square => item * item,
                    MonkeyOperation::Multiply(x) => item * x,
                    MonkeyOperation::Add(x) => item + x,
                };
                // Monkey inspected an item, so increase the inspect count.
                monkeys[m].inspect_count += 1;
                // Use the cooloff number to keep the worry levels manageable without
                // affecting the outcome of the calculcations. See comment further up.
                let newval = newval % cooloff_number;
                // Perform the test to determine the destination monkey.
                let dest = if newval % monkeys[m].test_divisor == 0 {
                    monkeys[m].true_dest
                } else {
                    monkeys[m].false_dest
                };
                // And send the item to that destination.
                monkeys[dest].items.push_back(newval);
            }
        }
    }

    // Finally, collect all the inspect counts.
    let mut inspect_counts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
    dbg!(&inspect_counts);
    // Sort them.
    inspect_counts.sort_unstable_by(|a, b| b.cmp(a));

    // Finally, calculate and print the level of monkey business.
    println!("Monkey business: {}", inspect_counts[0] * inspect_counts[1]);
}
