use std::collections::BTreeSet;
use std::sync::atomic::Ordering;

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
use crate::dpll::DECISION_COUNTER;

mod knowledge_base;
use crate::knowledge_base::{dedup_knowledge_base, Literal, Outcome};

mod heuristics;

#[cfg(test)]
mod tst;

fn main() {
    let example = get_example();
    let s = parse_sudoku(example);
    print_sudoku(&s);
    let a = sudoku_to_knowledge_base(&s);
    let a = dedup_knowledge_base(a);
    // println!("knowledge base:\n{}", a.draw());
    match run_dpll(a) {
        Outcome::Sat(ass) => {
            print_sudoku(&assigment_to_sudoku(&ass));
        }
        Outcome::Unsat => {
            println!("Unsatisfied");
        }
    }
    println!("num decisions: {}", DECISION_COUNTER.load(Ordering::Relaxed));
}

fn negate_literal((v, b): Literal) -> Literal {
    (v, !b)
}
