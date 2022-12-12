use std::fmt;

use super::{ExtendedChar, Node, NFA};

/* DEBUG TRAITS */

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "Node(n={})", self.n)
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for NFA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut delta = vec![];
        for (k, v) in &self.delta {
            // delta.push((k, v));
            delta.push(format!("{:?}->{:?}({:?})", k.0, v, k.1));
        }
        write!(
            f,
            "NFA(states={}, starting={:?}, delta={:?}, finished={:?})",
            self.states, self.starting, delta, self.finished
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
