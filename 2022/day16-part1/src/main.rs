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

    // Now check through all possible permutations of non-zero nodes using a recursive function.
    // We assume 'AA' has zero flow_rate (it should).
    let mut optimum: i32 = 0;
    generate_permutation(
        &nodes,
        &distances,
        &mut non_zero_nodes,
        &mut vec![('A', 'A')],
        30,
        0,
        &mut optimum,
    );

    println!("Optimal pressure release: {}", optimum);
}

fn generate_permutation(
    nodes: &HashMap<Name, Valve>,
    distances: &HashMap<(Name, Name), i32>,
    source: &mut Vec<Name>,
    dest: &mut Vec<Name>,
    time: i32,
    pressure: i32,
    optimum: &mut i32,
) {
    // Are we done here?
    // If there are only 2 units of time (or less) left we're done here.
    // We're obviously also done if the source is empty.
    if time <= 2 || source.is_empty() {
        // Check if this is better and store the optimal result.
        if pressure > *optimum {
            *optimum = pressure;
        }
        // Obviously, return early.
        return;
    }
    // Iterate through all elements remaining in the source.
    for i in 0..source.len() {
        // Remove the current element from the vector.
        let e = source.remove(i);

        // Determine the distance between the last two nodes.
        let add_dist = distances[&(*dest.last().unwrap(), e)];

        // Put the element onto the destination.
        dest.push(e);

        // Determine how much time will have passed until this node is ready.
        let new_time = time - (1 + add_dist);
        // Calculate how much pressure we save.
        let add_pressure = new_time * nodes[&e].flow_rate;

        // Finally, call recursively.
        generate_permutation(
            nodes,
            distances,
            source,
            dest,
            new_time,
            pressure + add_pressure,
            optimum,
        );

        // Remove the element from the destination.
        dest.pop();
        // And reinsert the element back into the vector, at the same precise location.
        source.insert(i, e);
    }
}

fn print_name_list(list: &Vec<Name>) {
    for n in list {
        print!("{}{}, ", n.0, n.1);
    }
    println!();
}
