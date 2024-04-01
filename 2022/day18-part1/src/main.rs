// Use a bespoke data structure for very fast access to the volume elements.
#[derive(Debug)]
struct Volume {
    data: Vec<bool>,
    dim: usize,
}

impl Volume {
    fn new(input: &Vec<[usize; 3]>) -> Volume {
        // Determine the largest index across all three dimensions.
        // Add 1 to it since it's an index and we're looking for its size.
        let dim = input.iter().flat_map(|a| a.iter()).max().unwrap() + 1;

        // Reserve the memory.
        let mut v = Volume {
            data: Vec::with_capacity(dim * dim * dim),
            dim,
        };

        // Fill the vector with `false` values.
        v.data.resize(dim * dim * dim, false);

        // Now fill the volume with all the known data.
        for [x, y, z] in input {
            *v.at_mut(*x, *y, *z) = true;
        }

        // Return the volume.
        v
    }

    fn at(&self, x: usize, y: usize, z: usize) -> &bool {
        &self.data[z * self.dim * self.dim + y * self.dim + x]
    }

    fn at_mut(&mut self, x: usize, y: usize, z: usize) -> &mut bool {
        &mut self.data[z * self.dim * self.dim + y * self.dim + x]
    }

    // Wrapper that allows invalid coordinates to simply return false.
    // This is consistent with the logic of the task.
    fn is_lava(&self, x: isize, y: isize, z: isize) -> bool {
        if x < 0 || y < 0 || z < 0 || x >= self.dim as isize || y >= self.dim as isize || z >= self.dim as isize {
            false
        } else {
            *self.at(x as usize, y as usize, z as usize)
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

    // Parse the input into a Vector of 3-tuples.
    let input = input
        .lines()
        .map(|l| {
            let l = l
                .split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            [l[0], l[1], l[2]]
        })
        .collect::<Vec<_>>();

    // Create the more efficient data structure.
    let v = Volume::new(&input);

    // Count the surface-area of each cube.
    let mut surface_area = 0;
    for [x,y,z] in &input {
        // Add 6 to the total for each cube, but ...
        surface_area += 6;
        // ... remove one for any adjacent cube.
        let ix = *x as isize;
        let iy = *y as isize;
        let iz = *z as isize;
        for [dx,dy,dz] in [
            [ix+1,iy,iz],
            [ix-1,iy,iz],
            [ix,iy+1,iz],
            [ix,iy-1,iz],
            [ix,iy,iz+1],
            [ix,iy,iz-1],
        ] {
            if v.is_lava(dx, dy, dz) {
                surface_area -= 1;
            }
        }
    }

    println!("Surface area: {}", surface_area);
}
