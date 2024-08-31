use std::collections::{BTreeMap, BTreeSet};
use crate::Id;

pub type Assignment = BTreeMap<Var, bool>;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Sat(Assignment),
    Unsat,
}

pub type Var = Id;
pub type Literal = (Var, bool);
pub type Clause = BTreeSet<Literal>;
pub type KnowledgeBase = Vec<Clause>;
