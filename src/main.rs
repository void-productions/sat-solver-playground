mod parse;

mod symbol;
pub use symbol::*;

use std::collections::BTreeSet;

#[derive(Debug)]
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
    loop {
        if c.contains(&Default::default()) { return Outcome::Unsat; }
        let n = c.len();
        c = step(c);
        if n == c.len() { return Outcome::Sat; }
    }
}

fn step(mut c: ClauseSet) -> ClauseSet {
    for v in vars(&c) {
        for p in clauses_with(v, true, &c) {
            for n in clauses_with(v, false, &c) {
                let mut new = p.clone();
                new.extend(n);
                c.insert(new);
            }
        }
    }
    c
}

fn set<T: Ord>(t: impl IntoIterator<Item=T>) -> BTreeSet<T> {
    t.into_iter().collect()
}

fn main() {
    let mut smap = SymbolMap::new();
    let a = smap.add("A".to_string());
    let b = smap.add("B".to_string());
    let c = set([
        set([(a, true)]),
        set([(b, false)]),
    ]);
    dbg!(run(c));
}
