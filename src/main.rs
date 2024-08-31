use std::collections::{BTreeMap, BTreeSet};

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

mod heuristics;
#[cfg(test)]
mod tst;

type Assignment = BTreeMap<Var, bool>;

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Sat(Assignment),
    Unsat,
}

pub type Var = Id;
pub type Literal = (Var, bool);
pub type Clause = BTreeSet<Literal>;
pub type KnowledgeBase = Vec<Clause>;

fn main() {
    let example = get_example();
    let s = parse_sudoku(example);
    print_sudoku(&s);
    let a = sudoku_to_knowledge_base(&s);
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
