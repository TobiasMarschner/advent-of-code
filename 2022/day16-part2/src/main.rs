use std::collections::{HashMap, VecDeque};

type Name = (char, char);

#[derive(Debug)]
struct Valve {
    name: (char, char),
    flow_rate: i32,
    tunnels: Vec<Name>,
}

impl Valve {
    fn print(&self) {
        print!("{}{} -- {:3} -- ", self.name.0, self.name.1, self.flow_rate);
        for t in &self.tunnels {
            print!("{}{}, ", t.0, t.1);
        }
        println!();
    }
}

struct GlobalState {
    nodes: HashMap<Name, Valve>,
    distances: HashMap<(Name, Name), i32>,
    source: Vec<Name>,
    dest_a: Vec<Name>,
    dest_b: Vec<Name>,
    optimum: i32,
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
    // Collect all nodes by name to a big map.
    let mut nodes: HashMap<Name, Valve> = HashMap::new();
    for line in input.lines() {
        // Split word-by-word.
        let words = line.split_whitespace().collect::<Vec<_>>();

        // Determine this node's name.
        let name = (
            words[1].chars().next().unwrap(),
            words[1].chars().nth(1).unwrap(),
        );

        // Grab the list of outgoing nodes for this node and strip the whitespace.
        // Returns "DD,II,BB".
        let tunnel_nodes = line
            .split("valve")
            .nth(1)
            .unwrap()
            .chars()
            .filter(|e| *e != ' ' && *e != 's')
            .collect::<String>();
        // Next, split by ','.
        // Returns ["DD", "II", "BB"].
        let tunnel_nodes = tunnel_nodes.split(',').collect::<Vec<_>>();
        // Turn the vector into a vector of names.
        // Returns [('D', 'D'), ('I', 'I'), ('B', 'B')].
        let tunnel_nodes: Vec<Name> = tunnel_nodes
            .iter()
            .map(|e| (e.chars().next().unwrap(), e.chars().nth(1).unwrap()))
            .collect();

        // Construct this node.
        let node = Valve {
            name,
            flow_rate: words[4]
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(';')
                .unwrap()
                .parse()
                .unwrap(),
            tunnels: tunnel_nodes,
        };
        // Add this node to the big map.
        nodes.insert(name, node);
    }

    // In order to properly calculate the optimal path and valve order
    // we need to first compute the cost getting from any node A to any
    // other node B, i.e. perform pathfinding.
    // We will precompute the results for faster lookup times later.
    let mut distances: HashMap<(Name, Name), i32> = HashMap::new();
    for an in nodes.keys() {
        // Technically we're performing Dijkstra's shortest path algorithm.
        // Since the edges all have weight 1 this devolves to simple breadth-first search.

        // Keep track of all nodes in a queue.
        let mut q: VecDeque<(Name, i32)> = VecDeque::new();
        // Add the current node to that queue.
        q.push_back((*an, 0));

        loop {
            // Queue empty? Then we're done.
            if q.is_empty() {
                break;
            }

            // Grab the next element (n) and its distance (d) from the queue.
            let (n, d) = q.pop_front().unwrap();

            // Since this is Dijkstra's algorithm, nodes are only visited once.
            // Therefore, add this node to the proper result.
            distances.insert((*an, n), d);

            // Next, look at all of this node's neighbors and add them to the queue.
            for neighbor in &nodes[&n].tunnels {
                // Only add them if they haven't already been visited.
                if distances.get(&(*an, *neighbor)).is_none() {
                    q.push_back((*neighbor, d + 1));
                }
            }
        }
    }

    // Keep track of all nodes with non-zero flow_rate.
    let mut non_zero_nodes: Vec<Name> = nodes
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(k, _)| *k)
        .collect();

    // Ensure each run is deterministic.
    non_zero_nodes.sort();

    // Create the GlobalState that is being passed through all iterations of the recursion.
    // We assume 'AA' has zero flow_rate (it should).
    let mut gs = GlobalState {
        nodes,
        distances,
        source: non_zero_nodes,
        dest_a: vec![('A', 'A')],
        dest_b: vec![('A', 'A')],
        optimum: 0,
    };

    // Now check through all possible permutations of non-zero nodes using a recursive function.
    generate_permutation(&mut gs, 26, 0, 0, 0);

    println!("Optimal pressure release: {}", gs.optimum);
}

fn generate_permutation(
    gs: &mut GlobalState,
    time_left: i32,
    busy_a: i32,
    busy_b: i32,
    pressure: i32,
) {
    // Our actors cannot be busy beyond the time limit.
    assert!(busy_a <= time_left);
    assert!(busy_b <= time_left);
    assert!(time_left >= 0);

    // Are we done here?
    // If there are only 2 units of time (or less) left we're done here.
    // We're obviously also done if the source is empty.
    if time_left <= 2 || gs.source.is_empty() {
        // Check if this is better and store the optimal result.
        if pressure > gs.optimum {
            print!("A dest: ");
            print_name_list(&gs.dest_a);
            print!("B dest: ");
            print_name_list(&gs.dest_b);
            println!("New Optimum: {}\n", pressure);
            gs.optimum = pressure;
        }
        // Obviously, return early.
        return;
    }

    // Cut criterion: If magically duplicating yourself, traveling to and opening all remaining valves doesn't
    // yield a result better than a previous optimum, this is a waste of time.
    let mut pressure_gain = 0;
    for e in gs.source.iter() {
        // Determine the distance between the last two nodes for both actors.
        let add_dist_a = gs.distances[&(*gs.dest_a.last().unwrap(), *e)];
        let add_dist_b = gs.distances[&(*gs.dest_b.last().unwrap(), *e)];

        // Determine the pressure gain from both actors' positions.
        let new_time_a = time_left - (1 + add_dist_a);
        let add_pressure_a = new_time_a * gs.nodes[&e].flow_rate;

        let new_time_b = time_left - (1 + add_dist_b);
        let add_pressure_b = new_time_b * gs.nodes[&e].flow_rate;

        // Add the bigger one, i.e. "clone" the actor that is closer to the node we're currently evaluating.
        // Moreover, only add pressure that actually contributes to the optimum.
        let gain = std::cmp::max(add_pressure_a, add_pressure_b);
        if gain >= 0 {
            pressure_gain += gain;
        }
    }
    // If the "duplicate yourself" pressure gain doesn't outperform the optimum there's no need to
    // keep going.
    if pressure + pressure_gain <= gs.optimum {
        // println!("Cut!");
        return;
    }

    // If both actors are busy, simply let time pass.
    if busy_a > 0 && busy_b > 0 {
        let pass = std::cmp::min(busy_a, busy_b);
        generate_permutation(gs, time_left - pass, busy_a - pass, busy_b - pass, pressure);
    } else if busy_a == 0 {
        // First actor is free to choose their next destination.
        for i in 0..gs.source.len() {
            // Remove the current element from the vector.
            let e = gs.source.remove(i);

            // Determine the distance between the last two nodes.
            let add_dist = gs.distances[&(*gs.dest_a.last().unwrap(), e)];

            // Put the element onto the destination.
            gs.dest_a.push(e);

            // Determine how much time will have passed until this node is ready.
            let new_time = time_left - (1 + add_dist);
            // Calculate how much pressure we save.
            let add_pressure = new_time * gs.nodes[&e].flow_rate;

            // Don't even bother with this node if there is nothing to be gained.
            if new_time > 0 {
                // Call recursively, but don't let any time pass.
                // If busy_b == 0, the next recursive step will choose for the other actor.
                // If busy_b > 0, the next recursive step will simply count down the time.
                generate_permutation(gs, time_left, 1 + add_dist, busy_b, pressure + add_pressure);
            }

            // Remove the element from the destination.
            gs.dest_a.pop();
            // And reinsert the element back into the vector, at the same precise location.
            gs.source.insert(i, e);
        }

        // OK, time to check one more thing: Doing nothing.
        // There is a possibility that the optimal strat for this actor is to twiddle their thumbs and
        // let the other actor handle all the remaining sources.
        generate_permutation(gs, time_left, time_left, busy_b, pressure);
    } else if busy_b == 0 {
        // Second actor is free to choose their next destination.

        // Iterate through all elements remaining in the source.
        for i in 0..gs.source.len() {
            // Remove the current element from the vector.
            let e = gs.source.remove(i);

            // Determine the distance between the last two nodes.
            let add_dist = gs.distances[&(*gs.dest_b.last().unwrap(), e)];

            // Put the element onto the destination.
            gs.dest_b.push(e);

            // Determine how much time will have passed until this node is ready.
            let new_time = time_left - (1 + add_dist);
            // Calculate how much pressure we save.
            let add_pressure = new_time * gs.nodes[&e].flow_rate;

            // Don't even bother with this node if there is nothing to be gained.
            if new_time > 0 {
                // Call recursively, but don't let any time pass.
                generate_permutation(gs, time_left, busy_a, 1 + add_dist, pressure + add_pressure);
            }

            // Remove the element from the destination.
            gs.dest_b.pop();
            // And reinsert the element back into the vector, at the same precise location.
            gs.source.insert(i, e);
        }

        // OK, time to check one more thing: Doing nothing.
        // There is a possibility that the optimal strat for this actor is to twiddle their thumbs and
        // let the other actor handle all the remaining sources.
        generate_permutation(gs, time_left, busy_a, time_left, pressure);
    }
}

fn print_name_list(list: &Vec<Name>) {
    for n in list {
        print!("{}{}, ", n.0, n.1);
    }
    println!();
}
