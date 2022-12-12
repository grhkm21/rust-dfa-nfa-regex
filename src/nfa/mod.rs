pub mod debug;
pub mod nfa;
pub mod utils;

pub use utils::*;

use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Node(pub usize);

pub struct NFA {
    states: usize,
    starting: HashSet<Node>,
    delta: HashMap<(Node, ExtendedChar), HashSet<Node>>,
    finished: HashSet<Node>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum ExtendedChar {
    Char(char),
    Wildcard,
}
