use crate::{negate_literal, Assignment, KnowledgeBase, Literal, Outcome};

pub fn run_dpll(knowledge_base: &KnowledgeBase) -> Outcome {
    // check satisfied
    if knowledge_base.is_empty() {
        return Outcome::Sat(Assignment::new());
    }
    // check unsatisfiable
    if knowledge_base.iter().any(|c| c.is_empty()) {
        return Outcome::Unsat;
    }

    let decision = get_decision(knowledge_base);
    if let Some(outcome) = recurse_dpll(knowledge_base, decision) {
        return outcome;
    }
    let negated_decision = negate_literal(decision);
    if let Some(outcome) = recurse_dpll(knowledge_base, negated_decision) {
        return outcome;
    }
    Outcome::Unsat
}

fn recurse_dpll(knowledge_base: &KnowledgeBase, negated_decision: Literal) -> Option<Outcome> {
    let new_knowledge_base = apply_decision(knowledge_base, negated_decision);
    if let Outcome::Sat(mut assignment) = run_dpll(&new_knowledge_base) {
        assignment.insert(negated_decision.0, negated_decision.1);
        return Some(Outcome::Sat(assignment));
    }
    None
}

fn get_decision(knowledge_base: &KnowledgeBase) -> Literal {
    *knowledge_base.iter().next().unwrap().iter().next().unwrap()
}

fn apply_decision(knowledge_base: &KnowledgeBase, decision: Literal) -> KnowledgeBase {
    let mut new_base = KnowledgeBase::new();
    let negated_decision = &&negate_literal(decision);
    for clause in knowledge_base {
        if !clause.contains(&decision) {
            let new_clause = clause.iter()
                                   .filter(|l| l != negated_decision)
                                   .cloned()
                                   .collect();
            new_base.insert(new_clause);
        }
    }
    new_base
}