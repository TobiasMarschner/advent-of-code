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
                        cave.0[y][x] = Tile::Rock;
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

// Grows upwards, bottom-layer starts at index 0.
struct Cave(Vec<[Tile; 7]>);

impl Cave {
    // Print the cave to stdout.
    // You can optionally provide a falling rock to print as well.
    #[allow(dead_code)]
    fn print(&self, falling_rock: &Option<FallingRock>) {
        for y in (0..(self.0.len() + 10)).rev() {
            print!("|");
            for x in 0..7 {
                // Air is the default tile.
                let mut tile = '.';
                // If the cave has data for this coordinate, check if it's a settled rock.
                if y < self.0.len() && self.0[y][x] == Tile::Rock {
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
        println!("+-------+\n");
    }

    // Checks if a given coordinate refers to a settled rock.
    // Also takes into account coordinates that extend beyond the current length of the Vec.
    fn is_rock(&self, x: usize, y: usize) -> bool {
        if y >= self.0.len() {
            false
        } else {
            self.0[y][x] == Tile::Rock
        }
    }

    // Vector not big enough? Ensure that it is big enough for the passed-along y-coordinate.
    fn extend_to(&mut self, y: usize) {
        // Simply push 7-element arrays of Air-tiles
        // onto the Vector until our condition is satisfied.
        while y >= self.0.len() {
            self.0.push([Tile::Air; 7]);
        }
    }

    // Returns the y-coordinate of the first rock-free line at the top of the tower.
    fn past_the_top(&self) -> usize {
        for (y, line) in self.0.iter().enumerate() {
            if line.iter().all(|e| e == &Tile::Air) {
                return y;
            }
        }
        // No elements in loop? Well, then the first free line is the first.
        0
    }

    // Count the number of Rock-tiles present in the cave.
    // We want to use this for debugging, since some rocks appear to get lost.
    #[allow(dead_code)]
    fn count_rocks(&self) -> usize {
        let mut total = 0;
        for line in &self.0 {
            for tile in line {
                if tile == &Tile::Rock {
                    total += 1;
                }
            }
        }
        total
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
    .cycle();

    // The cave where all the rocks will settle.
    let mut cave = Cave(Vec::new());

    // Simulate 2022 rocks.
    for _ in 0..2022 {
        // Create the next falling rock.
        let mut fr = Some(FallingRock {
            shape: *rock_shapes.next().unwrap(),
            // Always two spaces from the left wall.
            x: 2,
            // Always three lines of free space.
            y: cave.past_the_top() + 3,
        });

        // Keep moving l/r and down until the rock settles.
        loop {
            // Move left / right.
            let dir = input_directions.next().unwrap();
            fr = fr.unwrap().attempt_move(&mut cave, dir);

            // Next, move down.
            fr = fr.unwrap().attempt_move(&mut cave, FallingDirection::Down);
            // Did it settle? If so, move to the next rock.
            if fr.is_none() {
                break;
            }
        }
    }

    println!(
        "Topmost free y-coordinate after 2022 rocks have settled: {}",
        cave.past_the_top()
    );
}
