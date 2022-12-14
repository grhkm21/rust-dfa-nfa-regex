use char_stream::CharStream;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::ops::{BitAnd, BitOr};
use std::process::{Command, Stdio};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct NfaNode(pub usize);

#[derive(Clone)]
pub struct Nfa {
    states: usize,
    start_nodes: HashSet<NfaNode>,
    edges: HashMap<(NfaNode, ExtendedChar), HashSet<NfaNode>>,
    finish_nodes: HashSet<NfaNode>,
}

// TODO: Implement epsilon-moves
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum ExtendedChar {
    Char(char),
    Wildcard,
}

#[derive(Default)]
pub struct NfaExporter {
    args: Vec<String>,
    horizontal: bool,
}

/* DEBUG TRAITS */

impl fmt::Debug for NfaNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for Nfa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut delta = vec![];
        for (k, v) in &self.edges {
            delta.push(format!("{:?}->{:?}({:?})", k.0, v, k.1));
        }
        write!(
            f,
            "Nfa(states={}, starting={:?}, delta={:?}, finished={:?})",
            self.states, self.start_nodes, delta, self.finish_nodes
        )
    }
}
impl fmt::Debug for ExtendedChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Char(c) => *c,
                Self::Wildcard => '*',
            }
        )
    }
}

/* Nfa GENERIC FUNCTIONS */

impl Nfa {
    // Constructor
    pub fn new(
        states: usize,
        start_nodes: HashSet<NfaNode>,
        edges: HashMap<(NfaNode, ExtendedChar), HashSet<NfaNode>>,
        finish_nodes: HashSet<NfaNode>,
    ) -> Nfa {
        Nfa::reduce(Nfa {
            states,
            start_nodes,
            edges,
            finish_nodes,
        })
    }

    pub fn empty() -> Nfa {
        Nfa::new(1, [NfaNode(0)].into(), [].into(), [NfaNode(0)].into())
    }

    // Match CharStream with the Nfa
    // TODO: Connect this to NfaExporter to color path of match
    pub fn is_match(&self, stream: &mut CharStream) -> bool {
        let mut nodes: HashSet<NfaNode> = self.start_nodes.clone();
        for ch in stream {
            let mut new_nodes: HashSet<NfaNode> = HashSet::new();
            for &node in nodes.iter() {
                if let Some(set) = self.edges.get(&(node, ExtendedChar::Char(ch))) {
                    for &node in set.iter() {
                        new_nodes.insert(node);
                    }
                }
                if let Some(set) = self.edges.get(&(node, ExtendedChar::Wildcard)) {
                    for &node in set.iter() {
                        new_nodes.insert(node);
                    }
                }
            }
            nodes = new_nodes;
        }
        nodes.iter().any(|node| self.finish_nodes.contains(node))
    }

    // In regex term, transform a, b into a|b
    pub fn alternate(first: &Nfa, second: &Nfa) -> Nfa {
        let increase = |&node| {
            let NfaNode(n) = node;
            NfaNode(n + first.states)
        };
        let states = first.states + second.states;
        let start_nodes = first
            .start_nodes
            .union(&second.start_nodes.iter().map(increase).collect())
            .copied()
            .collect();
        let finish_nodes = first
            .finish_nodes
            .union(&second.finish_nodes.iter().map(increase).collect())
            .copied()
            .collect();

        let mut edges = first.edges.clone();

        for (&(NfaNode(n), ch), set) in second.edges.iter() {
            let set = set.iter().map(increase).collect();
            edges.insert((NfaNode(n + first.states), ch), set);
        }

        Nfa::new(states, start_nodes, edges, finish_nodes)
    }

    // In Regex term, transforms a, b into ab
    pub fn concat(first: &Nfa, second: &Nfa) -> Nfa {
        let states = first.states + second.states;
        let increase = |&node: &NfaNode| -> NfaNode {
            let NfaNode(n) = node;
            NfaNode(n + first.states)
        };
        let mut start_nodes = first.start_nodes.clone();
        if first
            .start_nodes
            .iter()
            .any(|&node| first.finish_nodes.contains(&node))
        {
            start_nodes = start_nodes
                .union(&second.start_nodes.iter().map(increase).collect())
                .copied()
                .collect();
        }

        let mut edges = first.edges.clone();
        let finish_nodes: HashSet<NfaNode> = second.finish_nodes.iter().map(increase).collect();
        let second_starting: HashSet<NfaNode> = second.start_nodes.iter().map(increase).collect();
        for (&(NfaNode(n), ch), set) in first.edges.iter() {
            let mut new_set: HashSet<NfaNode> = set.clone();
            if set.iter().any(|&node| first.finish_nodes.contains(&node)) {
                new_set = new_set.union(&second_starting).copied().collect();
            }
            edges.insert((NfaNode(n), ch), new_set);
        }

        second.edges.iter().for_each(|(&(node, ch), set)| {
            let new_set = set.iter().map(increase).collect();
            edges.insert((increase(&node), ch), new_set);
        });

        Nfa::new(states, start_nodes, edges, finish_nodes)
    }

    /* Reduction */

    // Ignores transition edge character
    pub fn get_node_edges(&self) -> HashMap<NfaNode, HashSet<NfaNode>> {
        let mut edges = HashMap::<NfaNode, HashSet<NfaNode>>::new();

        for (&(node, _), set) in self.edges.iter() {
            edges.entry(node).or_insert_with(HashSet::<NfaNode>::new);
            for &dest in set {
                edges.entry(node).and_modify(|s| {
                    s.insert(dest);
                });
            }
        }

        edges
    }

    pub fn reduce_in_place(&mut self) {
        // bfs from finish_nodes in reverse
        let mut queue = VecDeque::<NfaNode>::new();
        let mut seen = HashSet::<NfaNode>::new();

        // reverse node edges and start from end
        let mut node_edges = HashMap::<NfaNode, HashSet<NfaNode>>::new();
        for (node, set) in self.get_node_edges() {
            for dest in set {
                node_edges
                    .entry(dest)
                    .or_insert_with(HashSet::<NfaNode>::new);
                node_edges.entry(dest).and_modify(|s| {
                    s.insert(node);
                });
            }
        }

        for node in &self.finish_nodes {
            queue.push_back(*node);
            seen.insert(*node);
        }

        while let Some(head) = queue.pop_front() {
            if let Some(nodes) = node_edges.get(&head) {
                for &node in nodes {
                    if !seen.contains(&node) {
                        queue.push_back(node);
                        seen.insert(node);
                    }
                }
            }
        }

        print!("Reduced node count from {} to ", self.edges.len());
        for (_, val) in self.edges.iter_mut() {
            val.drain_filter(|node| !seen.contains(node));
        }
        self.edges.drain_filter(|_, v| v.is_empty());
        println!("{}!", self.edges.len());
    }

    pub fn reduce(nfa: Nfa) -> Nfa {
        let mut nfa = nfa;
        Nfa::reduce_in_place(&mut nfa);
        nfa
    }

    /* Custom constructors */

    // Accepting single character
    pub fn unit(ec: impl Into<ExtendedChar>) -> Nfa {
        let mut edges = HashMap::<(NfaNode, ExtendedChar), HashSet<NfaNode>>::new();
        let node0 = NfaNode(0);
        let node1 = NfaNode(1);
        edges.insert((node0, ec.into()), Into::<HashSet<NfaNode>>::into([node1]));
        Nfa::new(
            2,
            Into::<HashSet<NfaNode>>::into([node0]),
            edges,
            Into::<HashSet<NfaNode>>::into([node1]),
        )
    }

    // In regex term, transform a into a*
    pub fn star(nfa: &Nfa) -> Nfa {
        let start_nodes = nfa.start_nodes.clone();
        let mut finish_nodes = nfa.finish_nodes.clone();
        let mut edges = nfa.edges.clone();
        for (&(NfaNode(n), ch), set) in nfa.edges.iter() {
            let mut new_set = set.clone();
            if set.iter().any(|&node| nfa.finish_nodes.contains(&node)) {
                new_set = new_set.union(&nfa.start_nodes).copied().collect();
            }
            edges.insert((NfaNode(n), ch), new_set);
        }
        nfa.start_nodes.iter().for_each(|&NfaNode(n)| {
            finish_nodes.insert(NfaNode(n));
        });

        Nfa::new(nfa.states, start_nodes, edges.clone(), finish_nodes)
    }

    pub fn get_star(&self) -> Nfa {
        Nfa::star(self)
    }
}

impl BitOr for Nfa {
    type Output = Nfa;
    fn bitor(self, other: Nfa) -> Nfa {
        Nfa::alternate(&self, &other)
    }
}
impl BitOr for &'_ Nfa {
    type Output = Nfa;
    fn bitor(self, other: &Nfa) -> Nfa {
        Nfa::alternate(self, other)
    }
}
impl BitOr<&'_ Nfa> for Nfa {
    type Output = Nfa;
    fn bitor(self, other: &Nfa) -> Nfa {
        Nfa::alternate(&self, other)
    }
}
impl BitOr<Nfa> for &'_ Nfa {
    type Output = Nfa;
    fn bitor(self, other: Nfa) -> Nfa {
        Nfa::alternate(self, &other)
    }
}

impl BitAnd for Nfa {
    type Output = Nfa;
    fn bitand(self, other: Nfa) -> Nfa {
        Nfa::concat(&self, &other)
    }
}
impl BitAnd for &'_ Nfa {
    type Output = Nfa;
    fn bitand(self, other: &Nfa) -> Nfa {
        Nfa::concat(self, other)
    }
}
impl BitAnd<&'_ Nfa> for Nfa {
    type Output = Nfa;
    fn bitand(self, other: &Nfa) -> Nfa {
        Nfa::concat(&self, other)
    }
}
impl BitAnd<Nfa> for &'_ Nfa {
    type Output = Nfa;
    fn bitand(self, other: Nfa) -> Nfa {
        Nfa::concat(self, &other)
    }
}

impl NfaExporter {
    pub fn new() -> Self {
        NfaExporter {
            args: Vec::new(),
            horizontal: true,
        }
    }

    pub fn clear(&mut self) {
        self.args.clear();
    }

    pub fn set_horizontal(&mut self) {
        self.horizontal = true;
    }

    pub fn set_vertical(&mut self) {
        self.horizontal = false;
    }

    fn add_nfa_impl(&mut self, nfa: &Nfa, label: &str, boxed: bool) {
        let mut set_label = BTreeSet::<usize>::new();

        // Header
        self.args.push(format!("subgraph cluster_{label}{{"));

        if boxed {
            // Boxing digraph
            self.args.push(format!("label=\"{label}\";"));
            self.args.push("style=dotted;".to_string());
        }

        // Add transition edges
        for (&(NfaNode(u), ch), set) in nfa.edges.iter() {
            set_label.insert(u);
            for NfaNode(v) in set {
                set_label.insert(*v);
                self.args
                    .push(format!("{label}{u}->{label}{v}[label=\"{ch:?}\"];"));
            }
        }

        for u in set_label {
            let mut extra_args = " ".to_string();
            let in_start = nfa.start_nodes.contains(&NfaNode(u));
            let in_finish = nfa.finish_nodes.contains(&NfaNode(u));
            if in_start && in_finish {
                extra_args += "style=wedged,fillcolor=\"red:green\"";
            } else if in_start {
                extra_args += "color=\"red\"";
            } else if in_finish {
                extra_args += "color=\"green\"";
            }

            let attr = format!("{label}{u}[label=\"{u}\"{extra_args}];");
            self.args.push(attr);
        }

        self.args.push("}".to_string());
    }

    // Adds `dot` digraph contained within box to buffer
    // `label` must be unique for each nfa added
    pub fn add_nfa_boxed(&mut self, nfa: &Nfa, label: &str) {
        self.add_nfa_impl(nfa, label, true)
    }

    // Adds `dot` digraph to buffer without boxing it
    // `label` must be unique for each nfa added
    pub fn add_nfa_unboxed(&mut self, nfa: &Nfa, label: &str) {
        self.add_nfa_impl(nfa, label, false)
    }

    // For a single nfa, we directly dump
    pub fn dump_nfa(nfa: &Nfa, file_path: &str) -> Result<(), std::io::Error> {
        let mut exporter = Self::new();
        exporter.add_nfa_unboxed(nfa, "");
        exporter.dump(file_path)
    }

    pub fn dumps_nfa(nfa: &Nfa) -> String {
        let mut exporter = Self::new();
        exporter.add_nfa_unboxed(nfa, "");
        exporter.dumps()
    }

    // Dump buffer to file
    pub fn dump(&mut self, file_path: &str) -> Result<(), io::Error> {
        fs::write(file_path, self.dumps())
    }

    // Dump buffer to file in PNG format
    // TODO: Refactor code, unify error handling
    // TODO: Abstract this into separate file
    // TODO: Allow user to pass arguments
    // TODO: Support more file extensions
    pub fn dump_to_png(&mut self, file_path: &str) -> Result<(), io::Error> {
        let dumps = self.dumps();

        // Create `dot` process to create PNG
        let process = match Command::new("dot")
            .arg("-Tpng")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(err) => panic!("Couldn't spawn dot: {err}"),
            Ok(process) => process,
        };

        // Send input
        if let Err(err) = process.stdin.unwrap().write_all(dumps.as_bytes()) {
            panic!("Couldn't write to dot stdin: {err}");
        }

        // Read png
        // TODO: Optimize this to directly write to file_path
        let mut png_buf = Vec::new();
        if let Err(err) = process.stdout.unwrap().read_to_end(&mut png_buf) {
            panic!("Couldn't read dot stdout: {err}");
        }

        fs::write(file_path, png_buf)
    }

    // Returns dumped buffer
    pub fn dumps(&mut self) -> String {
        let mut str = "digraph{".to_string();
        str += &self.args.join("");
        if self.horizontal {
            str += "rankdir=\"LR\";"
        }
        str += "}";

        self.args = Vec::new();

        str
    }
}

impl From<char> for ExtendedChar {
    fn from(c: char) -> Self {
        ExtendedChar::Char(c)
    }
}
