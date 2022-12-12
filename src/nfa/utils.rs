use std::collections::{HashMap, HashSet};

use super::{ExtendedChar, Node, NFA};

/* ADDING NFA */

pub fn plus(first: &NFA, second: &NFA) -> NFA {
    let increase = |&node| {
        let Node(n) = node;
        Node(n + first.states)
    };
    let states = first.states + second.states;
    let starting = first
        .starting
        .union(&second.starting.iter().map(increase).collect())
        .copied()
        .collect();
    let finished = first
        .finished
        .union(&second.finished.iter().map(increase).collect())
        .copied()
        .collect();

    let mut delta = first.delta.clone();

    for (&(Node(n), ch), set) in second.delta.iter() {
        let set = set.iter().map(increase).collect();
        delta.insert((Node(n + first.states), ch), set);
    }

    NFA {
        states,
        starting,
        delta,
        finished,
    }
}

/* MULTIPLYING NFA */

pub fn times(first: &NFA, second: &NFA) -> NFA {
    let states = first.states + second.states;
    let increase = |&node: &Node| -> Node {
        let Node(n) = node;
        return Node(n + first.states);
    };
    let mut starting = first.starting.clone();
    if first
        .starting
        .iter()
        .any(|&node| first.finished.contains(&node))
    {
        starting = starting
            .union(&second.starting.iter().map(increase).collect())
            .copied()
            .collect();
    }
    // any nodes mapping to a first.finished state should map to second.starting states as well
    let mut delta = first.delta.clone();
    let finished: HashSet<Node> = second.finished.clone().iter().map(increase).collect();
    let second_starting: HashSet<Node> = second.starting.clone().iter().map(increase).collect();
    for (&(Node(n), ch), set) in first.delta.iter() {
        let mut new_set: HashSet<Node> = set.clone();
        if set.iter().any(|&node| first.finished.contains(&node)) {
            new_set = new_set.union(&second_starting).copied().collect();
        }
        delta.insert((Node(n), ch), new_set);
    }

    second.delta.iter().for_each(|(&(node, ch), set)| {
        let new_set = set.iter().map(increase).collect();
        delta.insert((increase(&node), ch), new_set);
    });

    NFA {
        states,
        starting,
        delta,
        finished,
    }
}

/* NFA BUILDING BLOCKS */

pub fn unit(ec: ExtendedChar) -> NFA {
    let mut delta = HashMap::<(Node, ExtendedChar), HashSet<Node>>::new();
    let node0 = Node(0);
    let node1 = Node(1);
    delta.insert((node0, ec), Into::<HashSet<Node>>::into([node1]));
    NFA {
        states: 2,
        starting: Into::<HashSet<Node>>::into([node0]),
        delta,
        finished: Into::<HashSet<Node>>::into([node1]),
    }
}

pub fn star(nfa: &NFA) -> NFA {
    let mut finished = nfa.finished.clone();
    let mut delta = nfa.delta.clone();
    for (&(Node(n), ch), set) in nfa.delta.iter() {
        let mut new_set = set.clone();
        if set.iter().any(|&node| nfa.finished.contains(&node)) {
            new_set = new_set.union(&nfa.starting).copied().collect();
        }
        delta.insert((Node(n), ch), new_set);
    }
    nfa.starting.iter().for_each(|&Node(n)| {
        finished.insert(Node(n));
    });

    NFA {
        states: nfa.states,
        starting: nfa.starting.clone(),
        delta,
        finished,
    }
}

pub fn empty() -> NFA {
    NFA {
        states: 1,
        starting: [Node(0)].into(),
        delta: [].into(),
        finished: [Node(0)].into(),
    }
}
