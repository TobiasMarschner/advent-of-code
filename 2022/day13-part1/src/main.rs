use std::{cmp::Ordering, iter::zip, str::Chars};

#[derive(Debug)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl Packet {
    // Parse a single packet line into the Packet data structure.
    fn from_string(s: &mut Chars) -> Vec<Packet> {
        // println!("Entering with {}", s.clone().collect::<String>());
        // We're going to iterate through this character by character.
        // It is assumed that the opening '[' is already stripped.
        // Create an empty vector packet to fill in a loop.
        let mut packet_list: Vec<Packet> = Vec::new();
        // Now iterate through all the remaining characters.
        loop {
            // Collect a single element, i.e. all characters until the first occurence of '[', ']' or ','.
            // The delimiter itself will be collected into `c`.
            let mut elem = String::new();
            let mut c: char;
            loop {
                c = s.next().unwrap();
                if c == '[' || c == ']' || c == ',' {
                    break;
                } else {
                    elem.push(c);
                }
            }
            // Parse the element if it isn't empty and add it to the list.
            if !elem.is_empty() {
                let num = Packet::Number(elem.parse().unwrap());
                packet_list.push(num);
            }

            // Encountering a new list?
            // Call recursively and collect everything there.
            if c == '[' {
                let sublist = Packet::from_string(s);
                packet_list.push(Packet::List(sublist));
            }

            // Encountered the end of the list?
            // Then we're done here.
            if c == ']' {
                break;
            }
        }
        packet_list
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

    // --- TASK BEGIN ---

    // Keep track of the results.
    let mut results: Vec<Ordering> = Vec::new();

    // Iterate through all packet-pairs and compare them.
    for packets in input.split("\n\n") {
        // Parse both of the packets.
        let packets: Vec<_> = packets
            .lines()
            .map(|x| Packet::from_string(&mut x[1..].chars()))
            .collect();

        // Actually compare both packets and record the result.
        results.push(packet_compare(&packets[0], &packets[1]));
    }

    // Finally, collect the sum of indices where packet_compare yielded Less.
    let magic_number = results.iter().enumerate().fold(0, |acc, (idx, e)| {
        if *e == Ordering::Less {
            acc + idx + 1
        } else {
            acc
        }
    });

    println!(
        "Sum of indices of correctly ordered packets: {}",
        magic_number
    );
}

fn packet_compare(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
    let mut result: Ordering;
    let mut zipper = zip(left, right);
    loop {
        match zipper.next() {
            Some(d) => {
                // Determine the Ordering result for the next two "elements", whatever they may be.
                result = match d {
                    (Packet::Number(lv), Packet::Number(rv)) => {
                        // Compare the integers directly.
                        lv.cmp(rv)
                    }
                    (Packet::Number(lv), Packet::List(rl)) => {
                        // Left is a number, right is a list.
                        // Convert the number to a list and then compare those.
                        let ll: Vec<Packet> = vec![Packet::Number(*lv)];
                        packet_compare(&ll, rl)
                    }
                    (Packet::List(ll), Packet::Number(rv)) => {
                        // Left is a list, right is a number.
                        // Convert the number to a list and then compare those.
                        let rl: Vec<Packet> = vec![Packet::Number(*rv)];
                        packet_compare(ll, &rl)
                    }
                    (Packet::List(ll), Packet::List(rl)) => {
                        // Recursively step into the lists.
                        packet_compare(ll, rl)
                    }
                };
            }
            None => {
                // Zipper empty?
                // We need to remember which of the lists ran out first.
                result = left.len().cmp(&right.len());
                // And definitely break, ofc, since the zipper is done.
                break;
            }
        }
        // Have we reached a conclusion already? If so, return.
        if result != Ordering::Equal {
            break;
        }
    }
    result
}
