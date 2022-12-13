use rust_dfa_regex::nfa::ExtendedChar as EChar;
use rust_dfa_regex::nfa::{Nfa, NfaExporter};

static STAR: fn(Nfa) -> Nfa = Nfa::star;
static UNIT: fn(EChar) -> Nfa = Nfa::unit;
static ALT: fn(Nfa, Nfa) -> Nfa = Nfa::alternate;
static CONCAT: fn(Nfa, Nfa) -> Nfa = Nfa::concat;

pub fn main() -> Result<(), std::io::Error> {
    // TARGET = ([0369]|[147][0369]*[258]|(([258]|[147][0369]*[147])([0369]|[258][0369]*[147])*([147]|[258][0369]*[258])))+

    // TARGET = PART PART*
    // PART = (PART1|PART2|(PART3_1 PART3_2* Part3_3))
    // PART1 = [0369]
    // PART2 = [147][0369]*[258]
    // PART3_1 = [258]|[147][0369]*[147]
    // PART3_2 = [0369]|[258][0369]*[147]
    // PART3_3 = [147]|[258][0369]*[258]

    let args = ('0'..='9')
        .map(|c| UNIT(EChar::Char(c)))
        .collect::<Vec<_>>();
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

    let r0369 = ALT(ALT(ALT(r0, r3), r6), r9);
    let r147 = ALT(ALT(r1, r4), r7);
    let r258 = ALT(ALT(r2, r5), r8);

    let part1 = r0369.clone();
    let part2 = CONCAT(CONCAT(r147.clone(), STAR(r0369.clone())), r258.clone());
    let r0369_star = STAR(r0369.clone());
    let part3_1 = ALT(
        r258.clone(),
        CONCAT(CONCAT(r147.clone(), r0369_star.clone()), r147.clone()),
    );
    let part3_2 = ALT(
        r0369.clone(),
        CONCAT(CONCAT(r258.clone(), r0369_star.clone()), r147.clone()),
    );
    let part3_3 = ALT(
        r147,
        CONCAT(CONCAT(r258.clone(), r0369_star.clone()), r258.clone()),
    );
    let part3 = CONCAT(
        CONCAT(part3_1.clone(), STAR(part3_2.clone())),
        part3_3.clone(),
    );
    let part = ALT(ALT(part1.clone(), part2.clone()), part3.clone());
    let mut target = ALT(part.clone(), STAR(part.clone()));

    // println!("div_3: {}", NfaExporter::dumps_nfa(target.clone()));
    // println!(
    //     "div_3 length: {}",
    //     NfaExporter::dumps_nfa(target.clone()).len()
    // );

    // target.reduce_in_place();
    // NfaExporter::dump_nfa(target.clone(), "out/div_3.dot")?;

    Ok(())
}
