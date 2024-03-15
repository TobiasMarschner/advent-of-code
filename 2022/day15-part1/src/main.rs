use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone)]
struct Sensor {
    sx: isize,
    sy: isize,
    bx: isize,
    by: isize,
}

impl Sensor {
    // Determine the distance between sensor and beacon, using Manhattan metric.
    fn beacon_distance(&self) -> isize {
        (self.sx - self.bx).abs() + (self.sy - self.by).abs()
    }

    // fn beacon_impossible(&self, x: isize, y: isize) -> bool {
    //     let d = (self.sx - x).abs() + (self.sy - y).abs();
    //     d <= self.beacon_distance()
    // }

    // Return a range for a given y-coordinate where no beacon could
    // possibly be present for this sensor.
    fn covered_in_line(&self, y: isize) -> Option<(isize, isize)> {
        let d = self.beacon_distance() - (self.sy - y).abs();
        if d < 0 {
            None
        } else {
            Some(((self.sx - d), (self.sx + d)))
        }
    }
}

// Collapse multiple ranges into the minimal set of ranges.
fn collapse_ranges(ranges: &mut VecDeque<(isize, isize)>) {
    // First, sort by minimal value..
    ranges
        .make_contiguous()
        .sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // Then continually merge pairs throughout the VecDeque.
    let mut idx: usize = 0;
    while idx + 1 < ranges.len() {
        // Grab the current pair.
        let a = ranges[idx];
        let b = ranges[idx + 1];
        // Check if the minimal vaue of b is contained in a.
        // In that case we can merge the two.
        if a.0 <= b.0 && b.0 <= a.1 {
            // Remove the first of the pair.
            ranges.remove(idx);
            // And update the second element.
            // (which just moved from idx+1 to idx)
            ranges[idx] = (a.0, a.1.max(b.1));
        } else {
            // If no merge took place, move on to the next pair.
            idx += 1;
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

    // --- TASK BEGIN ---

    // Parse the input.
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in input.lines() {
        let line = line
            .split('=')
            .skip(1)
            .map(|x| {
                x.chars()
                    .take_while(|c| *c != ',' && *c != ':')
                    .collect::<String>()
            })
            .map(|e| e.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        sensors.push(Sensor {
            sx: line[0],
            sy: line[1],
            bx: line[2],
            by: line[3],
        })
    }

    // The line to check.
    const Y: isize = 2000000;

    // Collect all the ranges in line Y where no beacons could be.
    let mut ranges: VecDeque<(isize, isize)> = VecDeque::new();
    for s in &sensors {
        let r = s.covered_in_line(Y);
        // Only collect non-empty ranges, ofc.
        if let Some(sr) = r {
            ranges.push_back(sr);
        }
    }

    // Next up, sort + collapse the ranges.
    collapse_ranges(&mut ranges);

    // Collect all beacons. (there are more sensors than beacons)
    let all_beacons: HashSet<(isize, isize)> = sensors.iter().map(|e| (e.bx, e.by)).collect();

    // Count the number of unique beacons in that line.
    let beacons_in_line = all_beacons.iter().filter(|e| e.1 == Y).count() as isize;

    // Calculate the total count within the line's ranges.
    let count = ranges.iter().fold(0, |acc, e| acc + (e.1 - e.0 + 1));

    println!(
        "No. of spots where no beacon can be: {}",
        count - beacons_in_line
    );
}
