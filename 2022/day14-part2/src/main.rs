use std::ops::RangeInclusive;

// Custom enum to represent the state of a tile.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
    Source,
}

struct TileMap {
    data: Vec<Tile>,
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    xsrc: isize,
    ysrc: isize,
    padding: isize,
}

impl TileMap {
    // Utilities for interfacing with the weird coordinates.
    fn width(&self) -> isize {
        self.xmax - self.xmin + 1 + self.padding * 2
    }
    fn height(&self) -> isize {
        self.ymax - self.ymin + 1 + self.padding * 2
    }
    fn yrange(&self) -> RangeInclusive<isize> {
        (self.ymin - self.padding)..=(self.ymax + self.padding)
    }
    fn xrange(&self) -> RangeInclusive<isize> {
        (self.xmin - self.padding)..=(self.xmax + self.padding)
    }

    fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        self.xmin - self.padding <= x
            && x <= self.xmax + self.padding
            && self.ymin - self.padding <= y
            && y <= self.ymax + self.padding
    }

    fn get(&self, x: isize, y: isize) -> Tile {
        // Make a bounds-check since not all invalid coordinates
        // are necessarily out-of-bounds of the vector.
        assert!(self.is_in_bounds(x,y));
        // Translate the task-coordinates to the actual 0..width / 0..height coordinates.
        let tx = x - (self.xmin - self.padding);
        let ty = y - (self.ymin - self.padding);
        let w = self.width();
        // Then perform the "fake" 2D access.
        self.data[(ty * w + tx) as usize]
    }

    fn set(&mut self, x: isize, y: isize, tile: Tile) {
        // Make a bounds-check since not all invalid coordinates
        // are necessarily out-of-bounds of the vector.
        assert!(self.is_in_bounds(x,y));
        // Translate the task-coordinates to the actual 0..width / 0..height coordinates.
        let tx = x - (self.xmin - self.padding);
        let ty = y - (self.ymin - self.padding);
        let w = self.width();
        // Then perform the "fake" 2D access.
        // dbg!(tx, ty, w);
        self.data[(ty * w + tx) as usize] = tile;
    }

    fn new(xmin: isize, xmax: isize, ymin: isize, ymax: isize, padding: isize) -> TileMap {
        // Make sure the bounds include the source.
        let xmin = xmin.min(500);
        let xmax = xmax.max(500);
        let ymin = ymin.min(0);
        let ymax = ymax.max(0);

        // Create an empty tilemap with all the parameters.
        let mut tm = TileMap {
            data: Vec::new(), // just temporarily
            xmin,
            xmax,
            ymin,
            ymax,
            xsrc: 500,
            ysrc: 0,
            padding,
        };
        // Actually allocate a Vector with appropriate size here.
        tm.data
            .resize((tm.width() * tm.height()) as usize, Tile::Air);
        // And set the source.
        tm.set(tm.xsrc, tm.ysrc, Tile::Source);
        tm
    }

    fn print(&self) {
        for y in self.yrange() {
            for x in self.xrange() {
                let c = match self.get(x, y) {
                    Tile::Air => '.',
                    Tile::Rock => '#',
                    Tile::Sand => 'o',
                    Tile::Source => '+',
                };
                print!("{c}");
            }
            println!();
        }
        println!();
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

    // We begin by parsing the input data.
    // Find the limits of the map.
    // Split by lines.
    let parsed_data = input.lines().collect::<Vec<_>>();
    // Then split by arrows within lines.
    let parsed_data: Vec<_> = parsed_data
        .iter()
        .map(|e| e.split(" -> ").collect::<Vec<_>>())
        .collect();
    // Then parse "503,4" into (503, 4).
    let parsed_data: Vec<_> = parsed_data
        .iter()
        .map(|l| {
            l.iter()
                .map(|e| e.split_once(',').unwrap())
                .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect();

    // Determine the limits.
    // We'll flatten the iterator here to reduce the 2D vector to 1D.
    let xmin = parsed_data.iter().flatten().map(|x| x.0).min().unwrap();
    let xmax = parsed_data.iter().flatten().map(|x| x.0).max().unwrap();
    let ymin = parsed_data.iter().flatten().map(|x| x.1).min().unwrap();
    let ymax = parsed_data.iter().flatten().map(|x| x.1).max().unwrap();

    println!("{},{},{},{}", &xmax, &xmin, &ymax, &ymin);

    // Create the TileMap with this info and a padding of 150.
    let mut tm = TileMap::new(xmin, xmax, ymin, ymax, 150);

    // NEXT UP: Create the rock formations based on the input data.
    for line in parsed_data {
        // Look at a sliding window of coordinate-pairs in every line.
        for ((ax, ay), (bx, by)) in line.windows(2).map(|p| (p[0], p[1])) {
            // The lines only iterate along one of the axes.
            if ax == bx {
                // The range is empty if start > end, so we're using a.min(b)..a.max(b) here.
                for y in ay.min(by)..=ay.max(by) {
                    tm.set(ax, y, Tile::Rock);
                }
            } else if ay == by {
                // The range is empty if start > end, so we're using a.min(b)..a.max(b) here.
                for x in ax.min(bx)..=ax.max(bx) {
                    tm.set(x, ay, Tile::Rock);
                }
            } else {
                panic!("Bad input");
            }
            // dbg!("(({},{}),({},{}))", ax, ay, bx, by);
            // tm.print();
        }
    }

    // PART TWO: Add the rock floor.
    for x in tm.xrange() {
        tm.set(x, tm.ymax + 2, Tile::Rock);
    }

    // NEXT UP: Actually simulate the sand falling.
    let mut total = 0;
    'rounds: loop {
        // Create a new sand particle at the source.
        let mut sand: (isize, isize) = (tm.xsrc, tm.ysrc);
        // Let it run its course.
        'single: loop {
            // Is this particle about to fall out of the map?
            if !tm.is_in_bounds(sand.0, sand.1 + 1) {
                // Since its running out into the void, the whole sim is done.
                break 'rounds;
            }
            // First, check directly underneath.
            if tm.get(sand.0, sand.1 + 1) == Tile::Air {
                sand = (sand.0, sand.1 + 1);
            // Next, check down-left.
            } else if tm.get(sand.0 - 1, sand.1 + 1) == Tile::Air {
                sand = (sand.0 - 1, sand.1 + 1);
            // down-right
            } else if tm.get(sand.0 + 1, sand.1 + 1) == Tile::Air {
                sand = (sand.0 + 1, sand.1 + 1);
            // All blocked? We're done with this particle then.
            } else {
                break 'single;
            }
        }
        // Afterwards, record it properly in the tilemap.
        tm.set(sand.0, sand.1, Tile::Sand);
        total += 1;

        // Is this sand particle at the source?
        if sand.0 == tm.xsrc && sand.1 == tm.ysrc {
            // The whole cave has run full (part two). Finish the sim.
            break 'rounds;
        }
    }

    tm.print();
    println!("Sand particles at rest: {}", total);
}
