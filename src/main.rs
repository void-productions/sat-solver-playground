#![allow(unused_imports)]

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
pub use dpll::*;

mod knowledge_base;
pub use knowledge_base::*;

mod heuristics;
pub use heuristics::*;

mod examples;
pub use examples::*;

#[cfg(test)]
mod tst;

fn main() -> std::io::Result<()> {
    let example = get_example("hard");
    let s = parse_sudoku(example);
    print_sudoku(&s);
    let a = sudoku_to_knowledge_base(&s);
    let a = dedup_knowledge_base(a);
    let mut state = State {
        knowledge_base: a,
        assignment: Assignment::new(),
    };
    state.simplify();
    let knowledge_base_json = knowledge_base_to_json(&state.knowledge_base);
    dump_json_to_file(&knowledge_base_json, "data/knowledge_base.json")?;
    // println!("knowledge base:\n{}", a.draw());
    match run_dpll(state.knowledge_base) {
        Outcome::Sat(ass) => {
            print_sudoku(&assigment_to_sudoku(&ass));
        }
        Outcome::Unsat => {
            println!("Unsatisfied");
        }
    }
    println!(
        "num decisions: {}",
        DECISION_COUNTER.load(Ordering::Relaxed)
    );

    Ok(())
}
