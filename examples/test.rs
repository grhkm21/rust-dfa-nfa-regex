use rust_dfa_regex::nfa::{ExtendedChar, Nfa, NfaExporter, NfaNode};

pub fn main() {
    let r0 = NfaNode(0);
    let r1 = NfaNode(1);
    let r2 = NfaNode(2);

    let nfa = &Nfa::new(
        3,
        [r0].into(),
        [((r0, ExtendedChar::Char('a')), [r1, r2].into())].into(),
        [r1, r2].into(),
    );

    let unit = &Nfa::unit(ExtendedChar::Char('b'));
    let nfa = &(nfa & unit);

    NfaExporter::dump_nfa(nfa, "out/dump-orig.dot").unwrap();
}
