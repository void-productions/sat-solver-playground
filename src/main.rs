#![allow(unused_imports)]

use std::collections::{HashMap, BTreeSet};
use std::sync::atomic::Ordering;
use std::hash::Hash;

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

mod cdcl;
pub use cdcl::*;

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
    let knowledge_base_json = knowledge_base_to_json(&a);
    dump_json_to_file(&knowledge_base_json, "data/knowledge_base.json")?;
    // println!("knowledge base:\n{}", a.draw());
    match run_cdcl(a) {
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
