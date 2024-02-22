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

    fn correct_tail(&mut self, head: Coord) {
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

// Move the whole rope towards a direction.
fn move_rope(rope: &mut Vec<Coord>, dir: Direction) {
    // Move the head in the given direction.
    rope.first_mut().unwrap().move_towards(dir);
    // Then iterate over all remaining "links" of the rope, starting from the head.
    for i in 1..rope.len() {
        // Copy the destination coordinate.
        let prev = rope[i-1];
        // And then correct the current link towards that previous one.
        rope[i].correct_tail(prev);
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

    // Keep track of the rope as a series of coordinates.
    let mut rope: Vec<Coord> = vec![Coord::new(); 10];

    // Keep track of all the coordnates visited by the tail.
    let mut visited_coordinates: HashSet<Coord> = HashSet::new();
    visited_coordinates.insert(*rope.last().unwrap());

    // Go through the instructions line-by-line.
    for line in input {
        // Split and parse each line into direction (char) and distance (i32).
        let s: Vec<_> = line.split(' ').collect();
        let direction = Direction::from_char(s[0].chars().next().unwrap());
        let distance = s[1].parse::<i32>().unwrap();

        // Then move the head `distance` times and correct the tail afterwards.
        for _ in 0..distance {
            // Move the entire rope along the given direction.
            move_rope(&mut rope, direction);
            // Finally, store the new tail coordinate in the set.
            visited_coordinates.insert(*rope.last().unwrap());
            // dbg!(&tail);
        }
    }

    println!("Number of visited coordinates: {}", visited_coordinates.len());
}

