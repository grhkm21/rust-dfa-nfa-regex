use char_stream::CharStream;
use std::collections::{HashMap, HashSet};

use rust_dfa_regex::nfa::{ExtendedChar, Nfa, NfaExporter, NfaNode};

const N: usize = 13;
pub fn main() -> Result<(), std::io::Error> {
    // Each state represents remainder mod 3
    let rs = (0..N).map(NfaNode).collect::<Vec<NfaNode>>();
    let chars = ('0'..='9').map(|c| c.into()).collect::<Vec<ExtendedChar>>();
    let mut edges = HashMap::<(NfaNode, ExtendedChar), HashSet<NfaNode>>::new();

    for i in 0..13 {
        for j in 0..10 {
            // i -> (i * 10 + j) % N
            let key = (rs[i], chars[j]);
            edges.insert(key, HashSet::<NfaNode>::new());
            edges.entry(key).and_modify(|s| {
                s.insert(rs[(i * 10 + j) % N]);
            });
        }
    }

    println!("Building NFA with {N} states...");
    let nfa = Nfa::new(N, [rs[0]].into(), edges, [rs[0]].into());

    for i in 1..=10000 {
        assert_eq!(
            i % N == 0,
            nfa.is_match(&mut CharStream::from(&i.to_string()))
        );
    }

    println!("Passed checks from 1 to 10000!");

    NfaExporter::dump_nfa(nfa, "out/div_13_states.dot")?;

    Ok(())
}
