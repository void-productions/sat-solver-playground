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

fn vars(c: &ClauseSet) -> BTreeSet<Var> {
    let mut out = BTreeSet::new();
    for x in c {
        for (v, _) in x {
            out.insert(*v);
        }
    }
    out
}

fn clauses_with(v: Var, b: bool, c: &ClauseSet) -> ClauseSet {
    let mut out = BTreeSet::new();
    for x in c {
        if x.iter().any(|y| y.0 == v && y.1 == b) {
            out.insert(x.clone());
        }
    }
    out.into_iter().collect()
}

fn run(mut c: ClauseSet) -> Outcome {
    c = filter(c);
    dump(&c, "A");
    loop {
        if c.contains(&Default::default()) { return Outcome::Unsat; }

        let c_old = c.clone();
        c = step(c);
        dump(&c, "B");
        c = filter(c);
        dump(&c, "C");
        if c_old == c { return Outcome::Sat; }
    }
}

fn step(c: ClauseSet) -> ClauseSet {
    let mut new_c = c.clone();
    for v in vars(&c) {
        for p in clauses_with(v, true, &c) {
            for n in clauses_with(v, false, &c) {
                let mut p_ = p.clone();
                p_.remove(&(v, true));
                let mut n_ = n.clone();
                n_.remove(&(v, false));

                let new = &p_ | &n_;
                // This heuristic conflicts with completeness, but seems to work in practice so far.
                if p.len() == 1 || n.len() == 1 || new.len() <= 2 {
                    new_c.insert(new);
                }
            }
        }
    }

    new_c
}

fn pos_vars(clause: &Clause) -> BTreeSet<Id> {
    clause.iter().filter(|(_, b)| *b).map(|(x, _)| *x).collect()
}

fn neg_vars(clause: &Clause) -> BTreeSet<Id> {
    clause.iter().filter(|(_, b)| !*b).map(|(x, _)| *x).collect()
}

fn filter(mut c: ClauseSet) -> ClauseSet {
    c.retain(|x| pos_vars(x).is_disjoint(&neg_vars(x)));
    c = c.iter()
         .cloned()
         .filter(|x|
            !c.iter().any(|y| y.is_subset(x) && y != x)
         ).collect();
    c
}

fn dump(c: &ClauseSet, s: &str) {
    println!("{}:\n", s);
    dbg!(c.len());
    let c2: ClauseSet = c.iter().filter(|x| x.len() == 1 && x.iter().next().unwrap().1).cloned().collect();
    println!("{}", c2.draw());
    dbg!(c2.len());
}

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
    dbg!(run(a));
}
