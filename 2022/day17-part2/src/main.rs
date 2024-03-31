use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RockShape {
    Minus,
    Plus,
    J,
    I,
    O,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FallingDirection {
    Left,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct FallingRock {
    shape: RockShape,
    // Coordinates of the falling rock identifying the bottom left piece of it.
    // For the '+'-shape this actually refers to a piece of air.
    // y counts up from the bottom layer 0 up to infinity.
    // x counts up from left 0 to right 6.
    x: usize,
    y: usize,
}

// To allow easy iteration over all coordinates of a falling rock
// I'm providing a custom iterator here.
#[derive(Debug)]
struct FallingRockIterator {
    fr: FallingRock,
    i: usize,
}

impl Iterator for FallingRockIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // The offsets for the different shapes, stored statically.
        let offsets: &'static [(usize, usize)] = match &self.fr.shape {
            RockShape::Minus => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            RockShape::Plus => &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            RockShape::J => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            RockShape::I => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            RockShape::O => &[(0, 0), (0, 1), (1, 0), (1, 1)],
        };
        // The actual calculation, where we apply rock coordinate + offset at current index i.
        // Yields None if i has already iterated over everything.
        if self.i < offsets.len() {
            // Don't forget to increment the offset index.
            let old_i = self.i;
            self.i += 1;
            Some((self.fr.x + offsets[old_i].0, self.fr.y + offsets[old_i].1))
        } else {
            // No incrementing the offset index once we've reached the end.
            None
        }
    }
}

impl FallingRock {
    // Construct a FallingRockIterator allowing us to iterate over all coordinates of this rock.
    fn iter(&self) -> FallingRockIterator {
        FallingRockIterator { fr: *self, i: 0 }
    }

    // Simulate the falling rock within the existing cave,
    // and attempt to move it in the specified direction.
    // Left and Right will either return a moved FallingRock,
    //   or return the exact same FallingRock if moving was not possible.
    // Down will either return a moved FallingRock or
    //   or consume the FallingRock and settle it as static Rock-tiles within the Cave.
    //
    // If the FallingRock has been settled, None will be returned.
    // Otherwise, the (possibly moved) Some(FallingRock) will be returned.
    fn attempt_move(mut self, cave: &mut Cave, dir: FallingDirection) -> Option<FallingRock> {
        match dir {
            FallingDirection::Left => {
                // In order to move the piece ...
                // (1) it must not touch the left wall
                // (2) all the tiles to the left of it must be air
                if self.x > 0 && self.iter().all(|(x, y)| !cave.is_rock(x - 1, y)) {
                    self.x -= 1;
                }
                Some(self)
            }
            FallingDirection::Right => {
                // In order to move the piece ...
                // (1) it must not touch the right wall
                // (2) all the tiles to the right of it must be air
                if self.iter().map(|e| e.0).max().unwrap() < 6
                    && self.iter().all(|(x, y)| !cave.is_rock(x + 1, y))
                {
                    self.x += 1;
                }
                Some(self)
            }
            FallingDirection::Down => {
                // If we've reached the bottom of the cave we have to settle.
                // If any of the tiles below the current piece are rocks, we also have to settle.
                if self.y == 0 || self.iter().any(|(x, y)| cave.is_rock(x, y - 1)) {
                    // Ensure the cave itself can hold all the possible coordinates.
                    // We're adding one extra b/c the topmost line should always be air-only.
                    cave.extend_to(self.y + 4);
                    // Actually settle the rock.
                    for (x, y) in self.iter() {
                        cave.set(x, y, Tile::Rock);
                    }
                    // Consume it.
                    None
                } else {
                    // Since we're not settling, we *are* moving.
                    self.y -= 1;
                    Some(self)
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
}

// Grows upwards, i.e. the "back" is the "top" of the stack.
#[derive(Debug, Clone)]
struct Cave {
    // The actual data - a double-ended queue / ring-buffer of lines.
    data: VecDeque<[Tile; 7]>,
    // In order to be able to compute to 1 trillion, as required for part 2,
    // we're going to regularly clean up "garbage" from the bottom of the tower
    // that is no longer needed for the simulation.
    // Nonetheless, we have to keep track of the current floor coordinate:
    floor: usize,
}

impl Cave {
    // Read an arbitrary coordinate within the cave.
    // This uses the simulated coordinates,
    //   i.e. the y coordinate can become *incredibly* large here.
    fn get(&self, x: usize, y: usize) -> Tile {
        self.data[y - self.floor][x]
    }

    // Write to an arbitrary coordinate within the cave.
    // This uses the simulated coordinates,
    //   i.e. the y coordinate can become *incredibly* large here.
    fn set(&mut self, x: usize, y: usize, val: Tile) {
        self.data[y - self.floor][x] = val;
    }

    // Get the simulated size of the tower.
    fn height(&self) -> usize {
        self.data.len() + self.floor
    }

    // Print the cave to stdout.
    // You can optionally provide a falling rock to print as well.
    #[allow(dead_code)]
    fn print(&self, falling_rock: &Option<FallingRock>) {
        for y in (self.floor..(self.height() + 10)).rev() {
            print!("|");
            for x in 0..7 {
                // Air is the default tile.
                let mut tile = '.';
                // If the cave has data for this coordinate, check if it's a settled rock.
                if y < self.height() && self.get(x, y) == Tile::Rock {
                    tile = '#';
                }
                // Next, check if a falling rock exists here and override the tile with '@' if so.
                if let Some(ref fr) = falling_rock {
                    if fr.iter().any(|(cx, cy)| cx == x && cy == y) {
                        tile = '@';
                    }
                }
                // Print the determined tile.
                print!("{}", tile);
            }
            println!("|");
        }
        if self.floor == 0 {
            println!("+-------+");
        }
        println!("\n");
    }

    // Checks if a given simulated coordinate refers to a settled rock.
    // Also takes into account coordinates that extend beyond the current length of the Vec.
    fn is_rock(&self, x: usize, y: usize) -> bool {
        if y >= self.height() {
            false
        } else {
            self.get(x, y) == Tile::Rock
        }
    }

    // Vector not big enough? Ensure that it is big enough for the passed-along y-coordinate.
    fn extend_to(&mut self, y: usize) {
        // Simply push 7-element arrays of Air-tiles
        // onto the Vector until our condition is satisfied.
        while y >= self.height() {
            self.data.push_back([Tile::Air; 7]);
        }
    }

    // Returns the simulated y-coordinate of the first rock-free line at the top of the tower.
    fn past_the_top(&self) -> usize {
        // Iterate through all data-lines, starting from the top.
        // Find the first line where there is at least one rock.
        // Then, return the y-coordinate that is one bigger, i.e. the previous, air-only line.
        // If no line could be found we assume we're at the start of simulation
        //   where y=0 is the first free line.
        self.data
            .iter()
            .enumerate()
            .rev()
            .find(|(_, line)| line.iter().any(|e| e == &Tile::Rock))
            .map_or(0, |(i, _)| i + 1 + self.floor)
    }

    // In order to pull off 1 trillion lines we have to regularly clean up "garbage"
    // at the bottom of the tower that isn't needed anymore for a correct simulation.
    // We do this by finding, starting from the top of the tower, the first two lines
    //   that, when "OR"ed together, yield a wall.
    //
    // Examples:
    //
    // Line A:      |##....#|   |#.#.#.#|   |##.....|
    // Line B:      |.#####.|   |.#.#.#.|   |...####|
    //
    // Yields:      |#######|   |#######|   |##.####|
    // Therefore:   OK          OK          CONTINUE
    //
    // Once located, anything below the lower of those two lines can be removed.
    // Once cut off the `floor` of the cave has to be incremented accordingly.
    //
    // Returns true if garbage was collected, false otherwise.
    fn collect_garbage(&mut self) -> bool {
        // Zip up a reverse iterator with another reverse-iterator that skips the topmost line
        //   to iterate over all pairs of lines.
        let dy = self
            .data
            .iter()
            .rev()
            .zip(self.data.iter().enumerate().rev().skip(1))
            .find(|(a, (_, b))| {
                // Find the first pair where every column contains at least one rock.
                a.iter()
                    .zip(b.iter())
                    .all(|(ea, eb)| ea == &Tile::Rock || eb == &Tile::Rock)
            })
            // We're interested in the coordinate, so map that out.
            .map(|(_, (y, _))| y);

        // Didn't find a wall? That's fine, no garbage to clean up then.
        if let Some(dy) = dy {
            // Drain the range from the bottom and drop the iterator.
            // This frees all the elements at the front of the VecDeque in bulk.
            drop(self.data.drain(0..dy));
            // Don't forget to adjust the floor.
            self.floor += dy;
            // Garbage was found and removed.
            true
        } else {
            // No garbage was found.
            false
        }
    }
}

// Store the *entire* state of the system in a struct.
// This includes the entire cave, its floor number and
// the current indices into the shape and direction iterators.
#[derive(Debug)]
struct SystemState {
    cave: Cave,
    rock_idx: usize,
    rocks_in_cave: usize,
    shape_idx: usize,
    dir_idx: usize,
}

impl SystemState {
    // Copy over the current state of the system and adjust the cave-data, deleting any empty
    // lines.
    fn new(cave: &Cave, rock_idx: usize, shape_idx: usize, dir_idx: usize) -> SystemState {
        // Clone the cave for inclusion in the SystemState.
        let mut cave = cave.clone();
        // To ensure consistency, cut off all empty lines at the top of the cave.
        let y = cave
            .data
            .iter()
            .enumerate()
            .rev()
            .find(|(_, line)| line.iter().any(|e| e == &Tile::Rock))
            .map(|(y, _)| y + 1)
            .unwrap();
        cave.data.truncate(y);
        // Collect statistics on the cave for faster comparison.
        let rock_count: usize = cave
            .data
            .iter()
            .map(|l| l.iter().filter(|e| e == &&Tile::Rock).count())
            .sum();
        // Create the new system state and return it.
        SystemState {
            cave,
            rock_idx,
            shape_idx,
            dir_idx,
            rocks_in_cave: rock_count,
        }
    }

    fn is_equal(&self, other: &Self) -> bool {
        // To improve performance, check the easy parameters first.
        self.rocks_in_cave == other.rocks_in_cave
            && self.shape_idx == other.shape_idx
            && self.dir_idx == other.dir_idx
            // Then, check the actual cave layout. For every line in both caves ...
            && self
                .cave
                .data
                .iter()
                .zip(other.cave.data.iter())
                // ... ensure every tile in each line is identical.
                .all(|(al, bl)| al.iter().zip(bl.iter()).all(|(ae, be)| ae == be))
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

    // Create an infinitely-looping iterator for the input directions.
    // We're also filtering out any characters that aren't '<' or '>' such as newlines
    //   and are simulatenously mapping '<' and '>' to FallingDirection::Left and ::Right respectively.
    let mut input_directions = input
        .chars()
        .filter_map(|e| match e {
            '<' => Some(FallingDirection::Left),
            '>' => Some(FallingDirection::Right),
            _ => None,
        })
        .enumerate()
        .cycle();

    // Also create an infinitely-looping iterator for the rock-types.
    let mut rock_shapes = [
        RockShape::Minus,
        RockShape::Plus,
        RockShape::J,
        RockShape::I,
        RockShape::O,
    ]
    .iter()
    .enumerate()
    .cycle();

    // The cave where all the rocks will settle.
    let mut cave = Cave {
        data: VecDeque::new(),
        floor: 0,
    };

    // The total collection of distinct states.
    // We have to find the loop in the system.
    let mut states: Vec<SystemState> = Vec::new();

    let mut current_dir_idx: usize;
    let mut current_shape_idx: usize;

    // We want to fast-forward *once*.
    let mut fast_forwarded = false;

    // Simulate ONE TRILLION rocks.
    let mut i = 0usize;
    const N: usize = 1_000_000_000_000usize;
    loop {
        // Grab the next shape.
        let (shape_idx, shape) = rock_shapes.next().unwrap();
        current_shape_idx = shape_idx;

        // Create the next falling rock.
        let mut fr = Some(FallingRock {
            shape: *shape,
            // Always two spaces from the left wall.
            x: 2,
            // Always three lines of free space.
            y: cave.past_the_top() + 3,
        });

        // Keep moving l/r and down until the rock settles.
        loop {
            // Grab the next direction.
            let (dir_idx, dir) = input_directions.next().unwrap();
            current_dir_idx = dir_idx;
            // Move left / right.
            fr = fr.unwrap().attempt_move(&mut cave, dir);

            // Next, move down.
            fr = fr.unwrap().attempt_move(&mut cave, FallingDirection::Down);
            // Did it settle? If so, move to the next rock.
            if fr.is_none() {
                break;
            }
        }

        // Attempt to collect garbage every cycle and
        // store the system state if garbage has been collected.
        // Only bother with fast-forwarding if we haven't forwarded already.
        if cave.collect_garbage() && !fast_forwarded {
            // Create the new SystemState.
            let s = SystemState::new(&cave, i, current_shape_idx, current_dir_idx);
            // Compare it against all old states.
            let res = states
                .iter()
                .rev()
                .find(|e| e.is_equal(&s));
            // Found the cycle? Excellent. Then fast-forward as much as we can.
            if let Some(res_elem) = res {
                // We know the indices and cave-makeup from then and now are exactly identicaly.
                // Only the rock_idx and floor-value are different.
                let rock_delta = s.rock_idx - res_elem.rock_idx;
                let floor_delta = s.cave.floor - res_elem.cave.floor;
                // Determine by how many rocks we can fast-forward to get as close to N as possible.
                let cycles_to_ff = (N - i) / rock_delta;
                // Then, actually fast-forward by that number of cycles.
                i += cycles_to_ff * rock_delta;
                cave.floor += cycles_to_ff * floor_delta;
                // Only fast-forward once.
                fast_forwarded = true;
            }
            states.push(s);
            // println!("No. of states: {}", states.len());
        }

        // Iterate the loop.
        i += 1;
        if i >= N {
            break;
        }
    }

    println!(
        "Topmost free y-coordinate after 2022 rocks have settled: {}",
        cave.past_the_top()
    );
}
