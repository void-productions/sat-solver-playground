use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use serde::Serialize;
use serde_json::json;
use crate::*;

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

#[derive(Serialize)]
struct Pair(usize, bool);

#[derive(Serialize)]
struct KnowledgeBaseDump {
    knowledge_base: Vec<Vec<Pair>>,
    mapping: Vec<(String, Id)>
}

pub fn knowledge_base_to_json(base: &KnowledgeBase) -> serde_json::Value {
    // Convert each BTreeSet of tuples to a BTreeSet of Pairs
    let json_vec: Vec<Vec<Pair>> = base
        .into_iter()
        .map(|set| set.into_iter().map(|(a, b)| Pair(a.0, *b)).collect())
        .collect();

    let mapping = symbol::get_gsymb_iter();

    let result: KnowledgeBaseDump = KnowledgeBaseDump {
        knowledge_base: json_vec,
        mapping,
    };

    // Convert to JSON value
    json!(result)
}

pub fn dump_json_to_file(json_value: &serde_json::Value, file_path: &str) -> std::io::Result<()> {
    // Open a file in write-only mode, create it if it doesn't exist
    let file = File::create(file_path)?;

    // Write the JSON value to the file using pretty format
    serde_json::to_writer_pretty(file, json_value)?;

    Ok(())
}

pub fn negate_literal((v, b): Literal) -> Literal {
    (v, !b)
}
