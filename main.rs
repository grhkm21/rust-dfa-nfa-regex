use nfa::*;
use std::fs;

pub fn main() {
    let r1 = Node(0);
    let r2 = Node(1);
    let r3 = Node(2);
    let nfa1 = NFA::new(
        3,
        [r1].into(),
        [
            ((r1, ExtendedChar::Char('x')), [r2].into()),
            ((r2, ExtendedChar::Char('y')), [r3].into()),
        ]
        .into(),
        [r3].into(),
    );
    let nfa2 = unit(nfa::ExtendedChar::Char('b'));
    let plus = plus(&nfa1, &nfa2);
    let times = times(&nfa1, &nfa2);
    println!("{:?}", nfa1);
    println!("{:?}", nfa2);
    println!("{:?}", plus);
    println!("{:?}", times);

    let mut dump = "".to_string();
    dump += "digraph{rankdir=\"LR\";";
    dump += &nfa1.dumps_boxed("nfa1");
    dump += &nfa2.dumps_boxed("nfa2");
    dump += &plus.dumps_boxed("plus");
    dump += &times.dumps_boxed("times");
    dump += "}";

    if let Err(e) = fs::write("out/dump.dot", dump) {
        println!("Error: {e}");
        println!("Are you running at crate root (not `src`)?")
    } else {
        println!("Dumped to out/dump.dot!");
        println!("Now try running `dot -Tpng out/dump.dot > out/dump.png`!");
    }
}
