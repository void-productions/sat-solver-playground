use crate::{KnowledgeBase, Literal};
use std::collections::BTreeSet;

type Heuristic = fn(knowledge_base: &KnowledgeBase, l: Literal) -> f32;

pub const USED_HEURISTIC: Heuristic = baseline;

pub fn baseline(knowledge_base: &KnowledgeBase, l: Literal) -> f32 {
    (knowledge_base.first().unwrap().first().unwrap() == &l) as i32 as f32
}

pub fn get_decision(knowledge_base: &KnowledgeBase) -> Literal {
    let variables: BTreeSet<_> = knowledge_base
        .iter()
        .map(|c| c.iter())
        .flatten()
        .map(|(v, _)| *v)
        .collect();

    let pos = variables.iter().map(|x| (*x, true));
    let neg = variables.iter().map(|x| (*x, false));
    let (result, _) = pos
        .chain(neg)
        .map(|x| (x, USED_HEURISTIC(knowledge_base, x)))
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap();
    result
}
