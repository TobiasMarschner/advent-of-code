use std::{cell::RefCell, cmp::Ordering, collections::BinaryHeap, rc::Rc};

struct Node {
    x: usize,
    y: usize,
    height: u8,
    outgoing: Vec<NodeRef>,
    best_dist: usize,
    previous: Option<NodeRef>,
}

// Make it easier to refer to Node references.
type NodeRef = Rc<RefCell<Node>>;

impl Node {
    fn new(x: usize, y: usize, height: u8) -> NodeRef {
        Rc::new(RefCell::new(Node {
            x,
            y,
            height,
            outgoing: Vec::new(),
            best_dist: usize::MAX,
            previous: None,
        }))
    }

    fn heuristic(&self, dx: usize, dy: usize) -> usize {
        // Manhattan distance
        let x_dist = ((self.x as isize) - (dx as isize)).unsigned_abs();
        let y_dist = ((self.y as isize) - (dy as isize)).unsigned_abs();
        x_dist + y_dist
    }

    fn to_visit_node(&self, best_est: usize) -> VisitNode {
        VisitNode {
            x: self.x,
            y: self.y,
            best_est,
        }
    }
}

// PartialEq can be implemented automatically.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct VisitNode {
    y: usize,
    x: usize,
    best_est: usize,
}

// I've opted to go with the coordinates instead of pointers
// to make it easier to achieve the total order.

// Adapted from the Rust docs on priority queues:
// https://doc.rust-lang.org/std/collections/binary_heap/index.html

// Manually implement Ord for VisitNode to ensure the queue becomes a min-heap.
// However, for y and x we keep the normal order.
impl Ord for VisitNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice the flipped order best_est here.
        other
            .best_est
            .cmp(&self.best_est)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

// For PartialOrd simply use the total order.
impl PartialOrd for VisitNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    // Understand the size of the map we're working with.
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    // Keep a complete map of all nodes, spatially distributed.
    // We first map lines, then columns, i.e. map[y][x].
    // Origin of the coordinate system is the top right.
    let mut map: Vec<Vec<NodeRef>> = Vec::with_capacity(height);

    // Also keep references to start and destination nodes.
    let mut start: Option<NodeRef> = None;
    let mut dest: Option<NodeRef> = None;

    // Parse the map.
    for (y, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            // Translate the character into the correct elevation.
            let elevation: u8 = match c {
                'S' => 0,
                'E' => 25,
                a => (a as u8) - b'a',
            };

            // Create the node.
            let node = Node::new(x, y, elevation);

            // Store starting and destination node.
            if c == 'S' {
                start = Some(node.clone());
            } else if c == 'E' {
                dest = Some(node.clone());
            }

            // Store the node in the global map.
            map.last_mut().unwrap().push(node.clone());
        }
    }

    // Store the coordinates of the destination for the heuristics later.
    let dest_x = dest.as_ref().unwrap().borrow().x;
    let dest_y = dest.as_ref().unwrap().borrow().y;

    // Create the neighbor relationship for all nodes, where applicable.
    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate() {
            // Grab a counted reference to the cell we're currently looking at.
            let mut current_node = map[y][x].borrow_mut();

            // Only create the following neighbor relationships if the heights allow it.

            // NORTH
            if y > 0 {
                let other_node = map[y - 1][x].clone();
                if current_node.height + 1 >= other_node.borrow().height {
                    current_node.outgoing.push(other_node);
                }
            }

            // SOUTH
            if y < height - 1 {
                let other_node = map[y + 1][x].clone();
                // Check the height difference.
                if current_node.height + 1 >= other_node.borrow().height {
                    current_node.outgoing.push(other_node);
                }
            }

            // WEST
            if x > 0 {
                let other_node = map[y][x - 1].clone();
                // Check the height difference.
                if current_node.height + 1 >= other_node.borrow().height {
                    current_node.outgoing.push(other_node);
                }
            }

            // EAST
            if x < width - 1 {
                let other_node = map[y][x + 1].clone();
                // Check the height difference.
                if current_node.height + 1 >= other_node.borrow().height {
                    current_node.outgoing.push(other_node);
                }
            }
        }
    }

    // Keep track of all nodes that need to be visited still.
    // We're going to use an efficient priority queue for this.
    let mut to_visit: BinaryHeap<VisitNode> = BinaryHeap::new();

    // Add the start node to that queue.
    let mut start_node = start.as_ref().unwrap().borrow_mut();
    // Set 0 as the current best distance.
    to_visit.push(start_node.to_visit_node(0));
    start_node.best_dist = 0;
    drop(start_node);

    // Finally, actually start the A* path finding algorithm.
    while let Some(current_vn) = to_visit.pop() {
        // Grab a mutable borrow to the actual node.
        let current_node = map[current_vn.y][current_vn.x].borrow_mut();

        // Have we reached the destination?
        // Then we're done here.
        if current_node.x == dest_x && current_node.y == dest_y {
            println!("Found the destination!");
            break;
        }

        // Now iterate through all neighbors.
        for nb in &current_node.outgoing {
            // Grab a mutable borrow to that neighbor.
            let mut nb = nb.borrow_mut();

            // Calculate the best known distance.
            let actual_dist = current_node.best_dist + 1;

            // Check if the computed distance is better than the previous optimum.
            if actual_dist < nb.best_dist {
                // Nice!
                // Update its distance.
                nb.best_dist = actual_dist;
                // Update its predecessor (point to us).
                nb.previous = Some(map[current_vn.y][current_vn.x].clone());
                // Add it to the priority queue.
                let heuristic = nb.heuristic(dest_x, dest_y);
                to_visit.push(nb.to_visit_node(actual_dist + heuristic));
            }
        }
    }

    print_solution(&map, dest.clone().unwrap());

    println!(
        "Minimal distance on destination: {}",
        dest.as_ref().unwrap().borrow().best_dist
    );
}

fn print_solution(map: &Vec<Vec<NodeRef>>, dest: NodeRef) {
    // Store the output.
    let mut output: Vec<Vec<char>> = Vec::new();

    // Recreate the input.
    for line in map {
        output.push(Vec::new());
        for nr in line {
            // Grab a borrow to the actual node.
            let node = nr.borrow();
            // Convert the height back into a character, lol.
            output.last_mut().unwrap().push((b'a' + node.height) as char);
        }
    }

    // Retrace the optimal path and replace the letters with arrows.
    let mut cn = dest;
    loop {
        // Overwrite the character with a #.
        output[cn.borrow().y][cn.borrow().x] = '#';
        if cn.borrow().previous.is_some() {
            let prev = cn.borrow().previous.clone().unwrap();
            cn = prev;
        } else {
            break;
        }
    }

    // Actually print.
    for line in output {
        for char in line {
            print!("{}", char);
        }
        println!();
    }
}
