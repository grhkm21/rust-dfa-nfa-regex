use rust_dfa_regex::nfa::{ExtendedChar, Nfa, NfaExporter, NfaNode};

pub fn main() -> Result<(), std::io::Error> {
    let r0 = NfaNode(0);
    let r1 = NfaNode(1);
    let r2 = NfaNode(2);
    let r3 = NfaNode(3);

    // x(?:yzx)*yz?
    let nfa1 = Nfa::new(
        4,
        [r0].into(),
        [
            ((r0, ExtendedChar::Char('x')), [r1].into()),
            ((r1, ExtendedChar::Char('y')), [r2, r3].into()),
            ((r2, ExtendedChar::Char('z')), [r0, r3].into()),
        ]
        .into(),
        [r3].into(),
    );

    let sub_nfa1 = Nfa::unit(ExtendedChar::Char('a'));
    let sub_nfa2 = Nfa::unit(ExtendedChar::Char('b'));

    // ab*
    let nfa2 = Nfa::concat(sub_nfa1.clone(), Nfa::star(sub_nfa2.clone()));

    // (x(?:yzx)*yz?)|(ab*)
    let plus = Nfa::alternate(nfa1.clone(), nfa2.clone());

    // (x(?:yzx)*yz?)|(ab*)
    let times = Nfa::concat(nfa1.clone(), nfa2.clone());

    // Initialize and dump to file
    let mut exporter = NfaExporter::new();
    exporter.set_horizontal();
    exporter.add_nfa_boxed(nfa1, "nfa1");
    exporter.add_nfa_boxed(nfa2, "nfa2");
    exporter.add_nfa_boxed(sub_nfa1, "sub_nfa1");
    exporter.add_nfa_boxed(sub_nfa2, "sub_nfa2");
    exporter.add_nfa_boxed(plus, "plus");
    exporter.add_nfa_boxed(times, "times");

    let e = exporter.dump("out/dump.dot");
    if e.as_ref().is_err() {
        println!("Error: {}", e.as_ref().unwrap_err());
        println!("Are you running from the crate root?");
    } else {
        println!("Successfully written to out/dump.dot!");
        println!("Now try running `dot -Tpng out/dump.dot > out/dump.png`!");
    }

    e
}
