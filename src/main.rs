mod parse;
pub use parse::*;

mod symbol;
pub use symbol::*;

mod draw;
pub use draw::*;

mod sudoku;
pub use sudoku::*;

#[cfg(test)]
mod tst;

use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Sat,
    Unsat,
}

pub type Var = Id;
pub type Literal = (Var, bool);
pub type Clause = BTreeSet<Literal>;
pub type ClauseSet = BTreeSet<Clause>;

fn main() {
    let s = "
-4- --- --5
568 -1- 4--
1-7 -54 -6-

--- --8 ---
73- 162 -98
685 47- -3-

4-- 8-- 25-
--- 7-6 3--
-26 3-5 --1

    ";
    let s = parse_sudoku(s);
    let a = sudoku_to_clauseset(&s);
}
