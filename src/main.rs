use std::collections::BTreeSet;

mod parse;
pub use parse::*;

mod symbol;
pub use symbol::*;

mod draw;
pub use draw::*;

mod sudoku;
pub use sudoku::*;

mod dpll;
use dpll::run_dpll;

mod knowledge_base;
use crate::knowledge_base::{Literal, Outcome};

mod heuristics;

#[cfg(test)]
mod tst;

fn main() {
    let example = get_example();
    let s = parse_sudoku(example);
    print_sudoku(&s);
    let a = sudoku_to_knowledge_base(&s);
    println!("knowledge base: {a}");
    match run_dpll(a) {
        Outcome::Sat(ass) => {
            print_sudoku(&assigment_to_sudoku(&ass));
        }
        Outcome::Unsat => {
            println!("Unsatisfied");
        }
    }
}

fn negate_literal((v, b): Literal) -> Literal {
    (v, !b)
}
