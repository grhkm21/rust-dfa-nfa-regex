# DN-Regex

## Project Aim

I wonder what DN stands for... In this project, I aim to implement conversion between DFA, NFA and Regex representations of regular expressions in Rust.

Based off of [@raaidrt/reg](https://github.com/raaidrt/reg/).

## TODO

#### Core Functionality

-   [ ] Implement DFA representation
-   [x] Implement NFA representation
-   [ ] Implement Regex representation

#### Representation Conversion

-   [ ] Implement NFA → DFA
-   [ ] Implement DFA → Regex
-   [ ] Implement Regex → DFA

#### Visualization

-   [x] Implement exporting NFA to `.dot`
    -   [ ] Add colors to start and end states
-   [ ] Implement exporting NFA to `.png`
-   [ ] Implement exporting DFA to `.dot`
-   [ ] Implement exporting DFA to `.png`

-   [ ] Abstract `dump`-related functions to separate struct
