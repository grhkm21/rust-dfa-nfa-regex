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
    -   [x] Add colors to start and end states
    -   [ ] Add option to color path after calling `.is_match(...)`
-   [ ] Implement exporting NFA to `.png`
-   [ ] Implement exporting DFA to `.dot`
-   [ ] Implement exporting DFA to `.png`

#### Code Base

-   [ ] Abstract `dump`-related functions to separate struct

#### Examples

-   [ ] Add examples for how to use all core functionalities (dump to files, etc.)
