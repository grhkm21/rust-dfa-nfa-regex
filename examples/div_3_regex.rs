use char_stream::CharStream;
use rust_dfa_regex::nfa::{Nfa, NfaExporter};

#[allow(clippy::redundant_clone)]
pub fn main() -> Result<(), std::io::Error> {
    // Reference: https://stackoverflow.com/a/24195550/16403001
    // TARGET = ([0369]|[147][0369]*[258]|(([258]|[147][0369]*[147])([0369]|[258][0369]*[147])*([147]|[258][0369]*[258])))+

    // TARGET = PART PART*
    // PART = (PART1|PART2|(PART3_1 PART3_2* Part3_3))
    // PART1 = [0369]
    // PART2 = [147][0369]*[258]
    // PART3_1 = [258]|[147][0369]*[147]
    // PART3_2 = [0369]|[258][0369]*[147]
    // PART3_3 = [147]|[258][0369]*[258]

    let args = ('0'..='9').map(Nfa::unit).collect::<Vec<_>>();
    let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9) = match &args[..] {
        [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9] => (
            r0.clone(),
            r1.clone(),
            r2.clone(),
            r3.clone(),
            r4.clone(),
            r5.clone(),
            r6.clone(),
            r7.clone(),
            r8.clone(),
            r9.clone(),
        ),
        _ => unreachable!(),
    };

    let r0369 = r0 | r3 | r6 | r9;
    let r147 = r1 | r4 | r7;
    let r258 = r2 | r5 | r8;

    let part1 = r0369.clone();
    let r0369_star = r0369.clone().get_star();
    let part2 = r147.clone() & r0369_star.clone() & r258.clone();
    let part3_1 = r258.clone() | (r147.clone() & r0369_star.clone() & r147.clone());
    let part3_2 = r0369.clone() | (r258.clone() & r0369_star.clone() & r147.clone());
    let part3_3 = r147.clone() | (r258.clone() & r0369_star.clone() & r258.clone());
    let part3 = part3_1.clone() & part3_2.clone().get_star() & part3_3.clone();
    let part = part1.clone() | part2.clone() | part3.clone();
    let nfa = part.clone() | part.clone().get_star();

    for i in 1..=10000 {
        assert_eq!(
            i % 3 == 0,
            nfa.is_match(&mut CharStream::from(&i.to_string()))
        );
    }

    println!("Passed checks from 1 to 10000!");

    NfaExporter::dump_nfa(nfa.clone(), "out/div_3_regex.dot")?;

    Ok(())
}
