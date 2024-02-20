// Custom data structure representing a single tree.
// We store its height and keep track from which cardinal directions it is visible.
#[derive(Debug, Copy, Clone)]
struct Tree {
    height: i8,
    visible_n: bool,
    visible_s: bool,
    visible_w: bool,
    visible_e: bool,
    viewdist_n: i8,
    viewdist_s: i8,
    viewdist_w: i8,
    viewdist_e: i8,
    scenic_score: i32,
}

impl Tree {
    fn new(height: i8) -> Tree {
        Tree {
            height,
            visible_n: true,
            visible_s: true,
            visible_w: true,
            visible_e: true,
            viewdist_n: 0,
            viewdist_s: 0,
            viewdist_w: 0,
            viewdist_e: 0,
            scenic_score: 0,
        }
    }

    // A tree is visible if it can be seen from at least one cardinal direction.
    fn visible(&self) -> bool {
        self.visible_n || self.visible_s || self.visible_w || self.visible_e
    }
}

// A custom struct for the whole forest.
#[derive(Debug)]
struct Forest {
    field: Vec<Tree>,
    dim: usize,
}

impl Forest {
    // Pretty printer for the forest, using terminal escape codes to color
    // the hidden trees bold and red.
    fn print(&self) {
        for y in 0..self.dim {
            for x in 0..self.dim {
                let tree = self.field[y * self.dim + x];
                if !tree.visible() {
                    print!("\x1b[1;31m");
                }
                print!("{}", tree.height);
                if !tree.visible() {
                    print!("\x1b[0m");
                }
            }
            println!();
        }
    }

    fn print_scenic_score(&self) {
        for y in 0..self.dim {
            for x in 0..self.dim {
                let tree = self.field[y * self.dim + x];
                // if !tree.visible() {
                //     print!("\x1b[1;31m");
                // }
                print!("{}", tree.scenic_score);
                // if !tree.visible() {
                //     print!("\x1b[0m");
                // }
            }
            println!();
        }
    }

    // Easy accessor for a tree using x and y coordintes.
    fn at(&mut self, x: usize, y: usize) -> &mut Tree {
        &mut self.field[y * self.dim + x]
    }

    // Easy accessor using x and y coordiantes that's allowed to fail
    // if the coordinates are out-of-bounds.
    fn ato(&mut self, x: isize, y: isize) -> Option<&mut Tree> {
        if x < 0 || y < 0 || x >= (self.dim as isize) || y >= (self.dim as isize) {
            None
        } else {
            Some(&mut self.field[(y * (self.dim as isize) + x) as usize])
        }
    }
}

fn main() {
    // Use command line arguments to specify the input filename.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Usage: ./main <input-file> <map-dimensions>\nNot enough arguments. Exiting.");
    }

    // Next, read the contents of the input file into a string for easier processing.
    let input = std::fs::read_to_string(&args[1]).expect("Error opening file");
    // Line-by-line processing is easiest.
    let input = input.lines();
    // Also get the dimension of the map.
    let dim = args[2].parse::<usize>().unwrap();

    // --- TASK BEGIN ---

    // First, parse the whole file into a two-dimensional array.
    let mut forest = Forest {
        field: Vec::with_capacity(dim * dim),
        dim,
    };

    // Simply iterate through all lines and characters.
    for line in input {
        for char in line.chars() {
            // Convert the character value into the respective number.
            forest.field.push(Tree::new(((char as u8) - b'0') as i8));
        }
    }

    // Now that we have the data, go through each row and column twice.
    // In essence we place an observer at the top and bottom of every column
    // and an observer at the east and west end of every row.
    // Then, we check which trees are visible for that observer,
    // recording the result in `VisibleDirections`.
    for i in 0..forest.dim {
        // Initialize the variables keeping track of the largest tree encountered along the way.
        let mut max_n: i8 = -1;
        let mut max_s: i8 = -1;
        let mut max_w: i8 = -1;
        let mut max_e: i8 = -1;

        for j in 0..forest.dim {
            // Get the current tree in this loop iteration as seen from the north.
            let tree_n = forest.at(i, j);
            // Check if that tree is obscured from view and update its visibility.
            if tree_n.height <= max_n {
                tree_n.visible_n = false;
            }
            // Update the largest recorded height.
            max_n = std::cmp::max(max_n, tree_n.height);

            // Now repeat the exact same steps for the other three directions.

            // SOUTH
            let tree_s = forest.at(i, forest.dim - j - 1);
            if tree_s.height <= max_s {
                tree_s.visible_s = false;
            }
            max_s = std::cmp::max(max_s, tree_s.height);

            // WEST
            let tree_w = forest.at(j, i);
            if tree_w.height <= max_w {
                tree_w.visible_w = false;
            }
            max_w = std::cmp::max(max_w, tree_w.height);

            // EAST
            let tree_e = forest.at(forest.dim - j - 1, i);
            if tree_e.height <= max_e {
                tree_e.visible_e = false;
            }
            max_e = std::cmp::max(max_e, tree_e.height);
        }
    }

    // Now, count the number of visible trees.
    let mut visible_count = 0;
    for x in 0..forest.dim {
        for y in 0..forest.dim {
            if forest.at(x, y).visible() {
                visible_count += 1;
            }
        }
    }

    // PART TWO
    // Calculate the visibility score for every tree.
    let mut best_scenic_score: i32 = 0;
    for x in 0..forest.dim {
        for y in 0..forest.dim {
            // Truly not the cleanest way to go about this.
            // Better would be an enum for all directions.
            // Iterate over all four cardinal directions.
            for dir in 0..4 {
                let current_height: i8 = forest.at(x, y).height;

                let mut walking_distance: isize = 1;
                loop {
                    // if x == 2 && y == 1 && dir == 2 {
                    //     println!("walkdist = {}", walking_distance);
                    //     dbg!(&forest.at(x, y));
                    // }
                    // Get the tree we're currently looking at.
                    // This depends on the direction we're currently looking at.
                    let tree = match dir {
                        0 => forest.ato(x as isize, (y as isize) - walking_distance), // north
                        1 => forest.ato(x as isize, (y as isize) + walking_distance), // south
                        2 => forest.ato((x as isize) + walking_distance, y as isize), // east
                        _ => forest.ato((x as isize) - walking_distance, y as isize), // west
                    };
                    match tree {
                        // Invalid coordinate? We're done already.
                        None => {
                            // if x == 2 && y == 1 && dir == 2 {
                            //     println!("NONE!");
                            //     println!("walkdist = {}", walking_distance);
                            //     dbg!(&forest.at(x, y));
                            // }
                            break;
                        }
                        // Something here? Check for its height.
                        Some(tree) => {
                            // We can see this tree, so add it to the count.
                            walking_distance += 1;
                            if tree.height >= current_height {
                                // Too tall? We're done counting then.
                                break;
                            }
                        }
                    }
                }

                // if x == 2 && y == 1 && dir == 2 {
                //     println!("walkdist = {}", walking_distance);
                //     dbg!(&forest.at(x, y));
                // }

                // Finally, set the tree distance.
                match dir {
                    0 => {
                        forest.at(x, y).viewdist_n = (walking_distance - 1) as i8;
                    }
                    1 => {
                        forest.at(x, y).viewdist_s = (walking_distance - 1) as i8;
                    }
                    2 => {
                        forest.at(x, y).viewdist_e = (walking_distance - 1) as i8;
                    }
                    _ => {
                        forest.at(x, y).viewdist_w = (walking_distance - 1) as i8;
                    }
                }
            }

            // Finally, calculate the tree's scenic score.
            let mut scenic_score: i32 = 1;
            scenic_score *= forest.at(x, y).viewdist_n as i32;
            scenic_score *= forest.at(x, y).viewdist_s as i32;
            scenic_score *= forest.at(x, y).viewdist_e as i32;
            scenic_score *= forest.at(x, y).viewdist_w as i32;
            forest.at(x, y).scenic_score = scenic_score;

            best_scenic_score = std::cmp::max(best_scenic_score, scenic_score);

            // println!("({},{},{})", x, y, scenic_score);
        }
    }

    // Print the forest's scenic scores and the best scenic score.
    forest.print_scenic_score();
    println!("Best scenic score: {}", best_scenic_score);
}
