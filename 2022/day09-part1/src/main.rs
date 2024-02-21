use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl Direction {
    fn from_char(dir: char) -> Direction {
        match dir {
            'R' => Right,
            'L' => Left,
            'U' => Up,
            'D' => Down,
            _ => {
                panic!("Invalid character to construct Direction form");
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new() -> Coord {
        Coord {
            x: 0,
            y: 0,
        }
    }

    fn move_towards(&mut self, dir: Direction) {
        match dir {
            Up => {
                self.y += 1;
            }
            Down => {
                self.y -= 1;
            }
            Left => {
                self.x -= 1;
            }
            Right => {
                self.x += 1;
            }
        }
    }

    fn correct_tail(&mut self, head: &Coord) {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        // Do nothing if there isn't any actual distance between
        // the tail and head, i.e. at least one of the dimensions
        // has distance value 2 or greater.
        if dx.abs() > 1 || dy.abs() > 1 {
            // Use the sign of the delta to move the tail towards the head.
            // This automatically takes care of the diagonal vs. horizontal vs. vertical
            // behvaior outlined in the task description.
            self.x += dx.signum();
            self.y += dy.signum();
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

    // One coordinate for head and tail each.
    let mut head = Coord::new();
    let mut tail = Coord::new();

    // Keep track of all the coordnates visited by the tail.
    let mut visited_coordinates: HashSet<Coord> = HashSet::new();
    visited_coordinates.insert(tail);

    // Go through the instructions line-by-line.
    for line in input {
        // Split and parse each line into direction (char) and distance (i32).
        let s: Vec<_> = line.split(' ').collect();
        let direction = Direction::from_char(s[0].chars().next().unwrap());
        let distance = s[1].parse::<i32>().unwrap();

        // Then move the head `distance` times and correct the tail afterwards.
        for _ in 0..distance {
            // Move the head along the given direction.
            head.move_towards(direction);
            // Then, correct the tail.
            tail.correct_tail(&head);
            // Finally, store the new coordinate in the set.
            visited_coordinates.insert(tail);
            // dbg!(&tail);
        }
    }

    println!("Number of visited coordinates: {}", visited_coordinates.len());
}

