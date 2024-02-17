use std::{cell::RefCell, fmt, rc::Rc};

// Custom data structure
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum NodeType {
    File,
    Directory,
}

use NodeType::*;

// #[derive(Debug)]
struct Node {
    node_type: NodeType,
    name: String,
    size: usize,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

// Manual implementation of Debug for Node.
// Necessary in order to remove the "parent" field from the output.
// Otherwise the printer will print indefinitely.
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("node_type", &self.node_type)
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    fn new(node_type: NodeType, name: &str, size: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            node_type,
            name: String::from(name),
            size,
            children: Vec::new(),
            parent: None,
        }))
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
    // Line-by-line processing is easiest.
    let input = input.lines();

    // --- TASK BEGIN ---

    // Set up our filesystem, starting with the root directory as the root node.
    let root = Node::new(Directory, "/", 0);

    // Also keep track of all directories in the graph separately.
    // This will help us actually determine the sum we're supposed to look for.
    let mut dir_list: Vec<Rc<RefCell<Node>>> = Vec::new();
    dir_list.push(root.clone());

    // Also set up a pointer for the current directory.
    let mut cd = root.clone();

    // Keep track whether we're currently reading directories.
    let mut ls_mode = false;

    // Read line by line
    for line in input {
        // Split any incoming line by spaces, makes our life easier down the line.
        let line = line.split(' ').collect::<Vec<_>>();

        // First of all, differentiate between command and list item.
        if line[0] == "$" {
            match line[1] {
                "cd" => {
                    // Changing directory.
                    ls_mode = false;
                    match line[2] {
                        "/" => {
                            // If the target is "/" switch back to the root directory.
                            cd = root.clone();
                        }
                        ".." => {
                            // Switch to parent directory.
                            // First, we need to borrow the actual Node of the current directory.
                            let cd_node = cd.borrow();
                            // Grab an Rc to the parent...
                            let pd = cd_node.parent.clone().unwrap();
                            drop(cd_node);
                            // ...and assign it to cd.
                            cd = pd;
                        }
                        dir => {
                            // In the standard case, look for the child-node with the correct name.
                            // Get from the Rc<RefCell<Node>> to the actual Node.
                            let cd_node = cd.borrow();
                            // From there, find the child directory with the matching name.
                            let target_dir = cd_node
                                .children
                                .iter()
                                .find(|x| x.borrow().name == dir)
                                .unwrap()
                                .clone();
                            // Drop the Ref<_, Node> to ensure we drop the borrow on cd.
                            drop(cd_node);
                            // Now assign the new "current directory".
                            cd = target_dir;
                        }
                    }
                }
                "ls" => {
                    ls_mode = true;
                }
                _ => (),
            }
        } else {
            // Looks like we're in list mode... right?
            assert!(ls_mode);

            // Assume it is a directory.
            let mut node_type = Directory;
            let mut size: usize = 0;
            let name = line[1];

            // If it is a file, update the size and type accordingly.
            if line[0] != "dir" {
                node_type = File;
                size = line[0].parse::<usize>().unwrap();
            }

            // Now create the node for this file / directory.
            let node = Node::new(node_type, name, size);
            // Update the children of the current directory.
            cd.borrow_mut().children.push(node.clone());
            // And ensure the new node points back to the current directory.
            node.borrow_mut().parent = Some(cd.clone());

            // Also add any new directories to the directory list.
            if node_type == Directory {
                dir_list.push(node.clone());
            }
        }
    }

    // All files and directories have been parsed into the data structure.
    // However, the size on all directories is currently 0.
    // Determine the size of all directories recursively using DFS.
    calc_node_size(root.clone());

    // Finally, actually perform what the task requested. (part one)
    // let mut total: usize = 0;
    // for dir in &dir_list {
    //     let size = dir.borrow().size;
    //     if size <= 100000 {
    //         total += size;
    //     }
    // }

    // Determine the amount of space we need to free.
    let to_free: usize = root.borrow().size - 40000000;
    let mut optimal_dir_size: usize = 70000000;
    for dir in &dir_list {
        let size = dir.borrow().size;
        // Dont' bother if the directory is too small.
        if size < to_free {
            continue;
        } else {
            // Looks like it's big enough.
            // Update our optimal result if it is the smalles we've encountered yet.
            optimal_dir_size = std::cmp::min(size, optimal_dir_size);
        }
    }

    println!("Result: {}", optimal_dir_size);
}

// Perform a depth-first-search on the tree in order to annotate the directory sizes.
// This method uses recursion to visit the entire tree.
fn calc_node_size(node: Rc<RefCell<Node>>) -> usize {
    let mut inner_node = node.borrow_mut();
    // If this node is a file, there's no recursion to be done.
    // Simply return the size of the file.
    if inner_node.node_type == File {
        inner_node.size
    } else {
        let mut size: usize = 0;
        // Iterate through all children and ...
        for child in &inner_node.children {
            // ... accumulate the size of all nodes recursively.
            size += calc_node_size(child.clone());
        }
        // One more thing: Update the size of this directory before returning.
        inner_node.size = size;
        size
    }
}
