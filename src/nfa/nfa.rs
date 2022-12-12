use char_stream::CharStream;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

use super::{ExtendedChar, Node, NFA};

/* NFA GENERIC FUNCTIONS */

impl NFA {
    // Constructor
    pub fn new(
        states: usize,
        starting: HashSet<Node>,
        delta: HashMap<(Node, ExtendedChar), HashSet<Node>>,
        finished: HashSet<Node>,
    ) -> NFA {
        NFA {
            states,
            starting,
            delta,
            finished,
        }
    }

    // Match CharStream with the NFA
    pub fn is_match(&self, stream: &mut CharStream) -> bool {
        let mut nodes: HashSet<Node> = self.starting.clone();
        for ch in stream {
            let mut new_nodes: HashSet<Node> = HashSet::new();
            for &node in nodes.iter() {
                if let Some(set) = self.delta.get(&(node, ExtendedChar::Char(ch))) {
                    for &new_node in set.iter() {
                        new_nodes.insert(new_node);
                    }
                }
                if let Some(set) = self.delta.get(&(node, ExtendedChar::Wildcard)) {
                    for &new_node in set.iter() {
                        new_nodes.insert(new_node);
                    }
                }
            }
            nodes = new_nodes;
        }
        nodes.iter().any(|node| self.finished.contains(node))
    }

    // Dumps `dot` digraph
    pub fn dumps(&self) -> String {
        let mut args = Vec::<String>::new();

        // Header
        args.push("digraph{".to_string());

        // Horizontal placement
        args.push("rankdir=\"LR\";".to_string());

        // Add transition edges
        for (&(Node(u), ch), set) in self.delta.iter() {
            for Node(v) in set {
                args.push(format!("{u}->{v}[label=\"{ch:?}\"];"));
            }
        }

        // Footer
        args.push("}".to_string());

        args.join("")
    }

    // Dumps `dot` digraph contained within box
    // Useful for combining multiple dumps
    pub fn dumps_boxed(&self, label: &str) -> String {
        let mut args = Vec::<String>::new();
        let mut set_label = BTreeSet::<usize>::new();

        // Header
        args.push(format!("subgraph cluster_{} {{", label));

        // Horizontal placement
        args.push(format!("label=\"{}\";", label));
        args.push("style=dotted;".to_string());

        // Add transition edges
        for (&(Node(u), ch), set) in self.delta.iter() {
            set_label.insert(u);
            for Node(v) in set {
                set_label.insert(*v);
                args.push(format!("{label}{u}->{label}{v}[label=\"{ch:?}\"];"));
            }
        }

        for u in set_label {
            args.push(format!("{label}{u} [label=\"{u}\"]"));
        }

        args.push("};\n".to_string());

        args.join("")
    }

    // Dumps `dot` digraph to given file
    pub fn dump(&self, file_path: &str) {
        let mut args = Vec::<String>::new();

        // Header
        args.push("digraph{".to_string());

        // Horizontal placement
        args.push("rankdir=\"LR\";".to_string());

        // Add transition edges
        for (&(Node(u), ch), set) in self.delta.iter() {
            for Node(v) in set {
                args.push(format!("{u}->{v}[label=\"{ch:?}\"];"));
            }
        }

        // Footer
        args.push("}".to_string());

        if let Err(_) = fs::write(file_path, args.join("")) {
            println!("Error: Failed to write to path {file_path}");
        }
    }
}
