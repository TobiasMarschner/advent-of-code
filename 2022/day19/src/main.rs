#[derive(Debug)]
struct Blueprint {
    // The id and costs parsed from the input.
    id: u16,
    ore_robot_ore_cost: u16,
    clay_robot_ore_cost: u16,
    obsidian_robot_ore_cost: u16,
    obsidian_robot_clay_cost: u16,
    geode_robot_ore_cost: u16,
    geode_robot_obsidian_cost: u16,
    // The maximal number of geodes that can be collected by this blueprint.
    // Initialized to 0 and overwritten by the solver, once it concludes.
    optimal_geode_count: u16,
}

impl Blueprint {
    // Solve the given blueprint using BFS.
    fn solve_bfs(&mut self, total_runtime: u16) {
        // For performance reasons we will search the solution space using breadth-first search.

        // vec_a has the RecursionStates for the current timeslot, while vec_b has the next slot's states.
        let mut vec_a: Vec<RecursionState> = Vec::with_capacity(2u64.pow(20) as usize);
        let mut vec_b: Vec<RecursionState> = Vec::with_capacity(2u64.pow(20) as usize);

        // Initialize the simulation with the starting state.
        vec_a.push(RecursionState {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        });

        // Iterate over all timeslots.
        // Building a robot at t=1 cannot influence the final geode-count,
        // so it's omitted from the simulation here.
        let mut early_exit = false;
        for ts in (2u16..=total_runtime).rev() {

            // Have we reached >= 2^20 elements on the input? Time to go for DFS instead.
            // Additionally, the queue-overhead shouldn't be worth it for the last few timesteps.
            if vec_a.len() >= 2u64.pow(20) as usize || ts <= 3 {
                // println!("Switching to recursive solving ...");
                // Iterate over all possibilities and run recursively.
                for rs in &vec_a {
                    self.solve_recursive(*rs, ts);
                }
                // And now we're done proper, no need to run the remaining loop iterations.
                early_exit = true;
                break;
            }

            // Process every RS of the past timeslot
            // to find all the states for the current timeslot.
            for rs in &vec_a {
                // Go through all five options and branch down them, if possible.
                // Specifically, we can either:
                // -> Build one of the four robot types, if resources permit.
                // -> Don't build anything at all.
                // It's important to note that the optimal solution may include waiting in the middle,
                //   i.e. letting resources accumulate so we can build one of the more expensive robots
                //   down the line instead of immediately spending the resources on a cheaper robot type.
                
                // Copy over the current state and let time for it pass.
                // This is the same no matter what type of robot we build since the robot will
                // go live at the end of the timeslot, not at its beginning.
                let mut next_rs = *rs;
                next_rs.ore += next_rs.ore_robots as u16;
                next_rs.clay += next_rs.clay_robots as u16;
                next_rs.obsidian += next_rs.obsidian_robots as u16;
                next_rs.geode += next_rs.geode_robots as u16;

                // Check whether we can build the different robots, using `rs` and not `next_rs`
                // since the resources have to be allocated at the beginning of the turn.

                // (1) Ore Robot
                if rs.ore >= self.ore_robot_ore_cost {
                    let mut nrs = next_rs;
                    nrs.ore -= self.ore_robot_ore_cost;
                    nrs.ore_robots += 1;
                    vec_b.push(nrs);
                }
                // (2) Clay Robot
                if rs.ore >= self.clay_robot_ore_cost {
                    let mut nrs = next_rs;
                    nrs.ore -= self.clay_robot_ore_cost;
                    nrs.clay_robots += 1;
                    vec_b.push(nrs);
                }
                // (3) Obsidian Robot
                if rs.ore >= self.obsidian_robot_ore_cost
                    && rs.clay >= self.obsidian_robot_clay_cost
                {
                    let mut nrs = next_rs;
                    nrs.ore -= self.obsidian_robot_ore_cost;
                    nrs.clay -= self.obsidian_robot_clay_cost;
                    nrs.obsidian_robots += 1;
                    vec_b.push(nrs);
                }
                // (4) Geode Robot
                if rs.ore >= self.geode_robot_ore_cost
                    && rs.obsidian >= self.geode_robot_obsidian_cost
                {
                    let mut nrs = next_rs;
                    nrs.ore -= self.geode_robot_ore_cost;
                    nrs.obsidian -= self.geode_robot_obsidian_cost;
                    nrs.geode_robots += 1;
                    vec_b.push(nrs);
                }
                // (5) Build nothing and let time pass.
                vec_b.push(next_rs);
            }

            // Done!
            // println!("Finished simulation round for t = {}", ts);
            // println!("      inserted elements: {}", vec_b.len());

            // Prune elements.
            prune_states(&mut vec_b, &mut vec_a);
            // println!("   elements after prune: {}", vec_a.len());

            // Clear vec_b since all the relevant states have been copied over to vec_a.
            vec_b.clear();
        }

        // Collect and print the final geode count.
        // Remember that we still have to simulate the geode-collection for t=1,
        // hence `e.geode + e.geode_robots as u16`.
        if !early_exit {
            self.optimal_geode_count = vec_a
                .iter()
                .map(|e| e.geode + e.geode_robots as u16)
                .max()
                .unwrap();
        }
        // println!("Found optimal geode count: {}", self.optimal_geode_count);
    }

    // Solve the task recursively, providing the current state and remaining time.
    // Essentially, and in contrast to `solve_bfs`, this recursive solver performs
    // depth-first-search (DFS) on the solution space instead of BFS.
    // This removes our ability to prune redundant elements, but doesn't require
    // keeping a queue of elements, making for a *much* lighter memory footprint.
    // Recommended for the final few timesteps.
    fn solve_recursive(&mut self, rs: RecursionState, t: u16) {
        // print!("t = {}, ", t);
        // rs.print();
        // println!();
        // Exit condition. If t == 1, we're basically done.
        // No need to build the final robot, it can't influence the final geode result.
        // Simply add one more round of harvesting (rs.geode_robots) and check for improvements.
        if t == 1 {
            let next_geode_count = rs.geode + rs.geode_robots as u16;
            if next_geode_count > self.optimal_geode_count {
                // Update the optimal result, if improved.
                self.optimal_geode_count = next_geode_count;
            }
            return;
        }

        // Check a cutoff-condition, in case this branch is not worth it.
        let upper_bound = rs.geode  // The resources we already have.
            // The resource the already existing robots would produce.
            + rs.geode_robots as u16 * t
            // The resources we would get if we produced one robot every timeslot.
            // This is the triangular number for (t - 1).
            + (t - 1) * t / 2;
        // Now check if this would be an improvement.
        if upper_bound <= self.optimal_geode_count {
            // No point continuing.
            return;
        }

        // The following section is basically the same as in `solve_bfs`.

        // Copy over the current state and let time for it pass.
        // This is the same no matter what type of robot we build since the robot will
        // go live at the end of the timeslot, not at its beginning.
        let mut next_rs = rs;
        next_rs.ore += next_rs.ore_robots as u16;
        next_rs.clay += next_rs.clay_robots as u16;
        next_rs.obsidian += next_rs.obsidian_robots as u16;
        next_rs.geode += next_rs.geode_robots as u16;

        // Check whether we can build the different robots, using `rs` and not `next_rs`
        // since the resources have to be allocated at the beginning of the turn.

        // (1) Ore Robot
        if rs.ore >= self.ore_robot_ore_cost {
            let mut nrs = next_rs;
            nrs.ore -= self.ore_robot_ore_cost;
            nrs.ore_robots += 1;
            self.solve_recursive(nrs, t - 1);
        }
        // (2) Clay Robot
        if rs.ore >= self.clay_robot_ore_cost {
            let mut nrs = next_rs;
            nrs.ore -= self.clay_robot_ore_cost;
            nrs.clay_robots += 1;
            self.solve_recursive(nrs, t - 1);
        }
        // (3) Obsidian Robot
        if rs.ore >= self.obsidian_robot_ore_cost && rs.clay >= self.obsidian_robot_clay_cost {
            let mut nrs = next_rs;
            nrs.ore -= self.obsidian_robot_ore_cost;
            nrs.clay -= self.obsidian_robot_clay_cost;
            nrs.obsidian_robots += 1;
            self.solve_recursive(nrs, t - 1);
        }
        // (4) Geode Robot
        if rs.ore >= self.geode_robot_ore_cost && rs.obsidian >= self.geode_robot_obsidian_cost {
            let mut nrs = next_rs;
            nrs.ore -= self.geode_robot_ore_cost;
            nrs.obsidian -= self.geode_robot_obsidian_cost;
            nrs.geode_robots += 1;
            self.solve_recursive(nrs, t - 1);
        }
        // (5) Build nothing and let time pass.
        self.solve_recursive(next_rs, t - 1);
    }
}

// Store all of the state that's passed up and down the recursion in one struct.
// Derive PartialOrd + Ord for lexicographic sorting, something we'll use during pruning.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RecursionState {
    // The currently active fleet of robots.
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
    // Our resources.
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl RecursionState {
    #[allow(dead_code)]
    fn print(&self) {
        print!("{:>3} OR, ", self.ore_robots);
        print!("{:>3} CR, ", self.clay_robots);
        print!("{:>3} BR, ", self.obsidian_robots);
        print!("{:>3} GR, ", self.geode_robots);
        print!("{:>3} O, ", self.ore);
        print!("{:>3} C, ", self.clay);
        print!("{:>3} B, ", self.obsidian);
        print!("{:>3} G, ", self.geode);
    }
}

// Copy over states for the next simulation round, pruning a lot of (but not all) 
//   RecursionStates that are "strictly inferior" in terms of Pareto optimality.
// Will clear any previously present elements in `dest`.
//
// A few words on the general idea here:
// The goal here is to check for Pareto improvements. An example:
//   RS1: 2 ore, 2 clay, 1 ore robot, 1 clay robot, 20 minutes left
//   RS2: 2 ore, 1 clay, 1 ore robot, 1 clay robot, 20 minutes left
// RS1 is just as "good" in terms of ore, robot counts and time left
//   but is "strictly better" in terms of clay. It makes no sense to continue
//   running the simulation for RS2 b/c it cannot possibly produce a better
//   outcome than RS1.
// Having more resources, robots or time can only ever lead to better outcomes.
// If, however, RS1 had, say, one more ore but less clay than RS2 we cannot say
//   that RS1 is "strictly better". It is different, having made a different tradeoff in
//   resource collection, which may or may not lead to a better outcome overall.
// This comparison is used to cut off redundant simulation paths in the solver.
fn prune_states(source: &mut [RecursionState], dest: &mut Vec<RecursionState>) {
    // Begin by sorting the source lexicrgraphically and clearing the destination.
    source.sort_unstable();
    dest.clear();
    // Iterate through it from smallest to largest element and look at every pair of states.
    for (a, b) in source.iter().zip(source.iter().skip(1)) {
        // Don't copy a over if it is strictly inferior or equal to b.
        // Compare from bottom to top.
        if a.geode > b.geode
            || a.obsidian > b.obsidian
            || a.clay > b.clay
            || a.ore > b.ore
            || a.geode_robots > b.geode_robots
            || a.obsidian_robots > b.obsidian_robots
            || a.clay_robots > b.clay_robots
            || a.ore_robots > b.ore_robots
        {
            dest.push(*a);
        }
    }
    // Copy over the very last element, too.
    dest.push(*source.last().unwrap());
}

fn main() {
    // Use command line arguments to specify the input filename.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: ./main <input-file>\nNo input file provided. Exiting.");
    }

    // Next, read the contents of the input file into a string for easier processing.
    let input = std::fs::read_to_string(&args[1]).expect("Error opening file");

    // First, parse all the blueprints.
    let mut blueprints = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .map(|l| Blueprint {
            id: l[1].trim_end_matches(':').parse::<u16>().unwrap(),
            ore_robot_ore_cost: l[6].parse::<u16>().unwrap(),
            clay_robot_ore_cost: l[12].parse::<u16>().unwrap(),
            obsidian_robot_ore_cost: l[18].parse::<u16>().unwrap(),
            obsidian_robot_clay_cost: l[21].parse::<u16>().unwrap(),
            geode_robot_ore_cost: l[27].parse::<u16>().unwrap(),
            geode_robot_obsidian_cost: l[30].parse::<u16>().unwrap(),
            optimal_geode_count: 0,
        })
        .collect::<Vec<_>>();

    // PART ONE

    // Solve for every blueprint with time 24.
    for bp in &mut blueprints {
        // Solve every blueprint with TOTAL_RUNTIME minutes of time.
        // println!("Solving Blueprint {}", bp.id);
        bp.solve_bfs(24u16);
    }

    println!(
        "Total Quality Level for Part 1: {}",
        blueprints
            .iter()
            .map(|b| b.id * b.optimal_geode_count)
            .sum::<u16>()
    );

    // PART TWO
    
    // Now solve the first three blueprints again, but for 32 minutes.
    for bp in blueprints.iter_mut().take(3) {
        bp.solve_bfs(32u16);
    }

    println!(
        "Multiplied Geode Counts for Part 2: {}",
        blueprints
            .iter()
            .take(3)
            .map(|b| b.optimal_geode_count as u64)
            .product::<u64>()
    );
}
