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

    let r = ('0'..='9').map(Nfa::unit).collect::<Vec<_>>();

    let r0369 = &(&r[0] | &r[3] | &r[6] | &r[9]);
    let r147 = &(&r[1] | &r[4] | &r[7]);
    let r258 = &(&r[2] | &r[5] | &r[8]);

    let part1 = r0369;
    let r0369_star = &r0369.get_star();
    let part2 = &(r147 & r0369_star & r258);
    let part3_1 = &(r258 | (r147 & r0369_star & r147));
    let part3_2 = &(r0369 | (r258 & r0369_star & r147));
    let part3_3 = &(r147 | (r258 & r0369_star & r258));
    let part3 = &(part3_1 & part3_2.get_star() & part3_3);
    let part = &(part1 | part2 | part3);
    let nfa = part | part.get_star();

    for i in 1..=10000 {
        assert_eq!(
            i % 3 == 0,
            nfa.is_match(&mut CharStream::from(&i.to_string()))
        );
    }

    println!("Passed checks from 1 to 10000!");

    NfaExporter::dump_nfa(&nfa, "out/div_3_regex.dot")?;

    Ok(())
}
