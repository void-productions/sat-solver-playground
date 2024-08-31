use crate::{negate_literal, Literal};
use std::collections::BTreeSet;
use crate::knowledge_base::KnowledgeBase;

type Heuristic = fn(knowledge_base: &KnowledgeBase, l: Literal) -> f32;

pub const USED_HEURISTIC: Heuristic = fractional;

pub fn baseline(knowledge_base: &KnowledgeBase, l: Literal) -> f32 {
    (knowledge_base.first().unwrap().first().unwrap() == &l) as i32 as f32
}

pub fn fractional(knowledge_base: &KnowledgeBase, l: Literal) -> f32 {
    let neg_l = negate_literal(l);
    let mut sum: f32 = 0.0;
    if !l.1 {
        return -1000.0;
    }
    for clause in knowledge_base {
        if clause.contains(&neg_l) || clause.contains(&l) {
            sum += 1.0 / clause.len() as f32;
        }
    }
    -sum
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
