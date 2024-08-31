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

pub fn dedup_knowledge_base(knowledge_base: KnowledgeBase) -> KnowledgeBase {
    let mut visited = BTreeSet::new();
    let mut deduped_base = KnowledgeBase::new();
    for c in knowledge_base {
        if !visited.contains(&c) {
            visited.insert(c.clone());
            deduped_base.push(c);
        }
    }
    deduped_base
}
