use crate::{negate_literal, Assignment, KnowledgeBase, Literal, Outcome};

pub fn run_dpll(knowledge_base: KnowledgeBase) -> Outcome {
    let knowledge_base = simplify(knowledge_base);

    // check satisfied
    if knowledge_base.is_empty() {
        return Outcome::Sat(Assignment::new());
    }
    // check unsatisfiable
    if knowledge_base.iter().any(|c| c.is_empty()) {
        return Outcome::Unsat;
    }

    let decision = get_decision(&knowledge_base);
    if let Some(outcome) = recurse_dpll(&knowledge_base, decision) {
        return outcome;
    }
    let negated_decision = negate_literal(decision);
    if let Some(outcome) = recurse_dpll(&knowledge_base, negated_decision) {
        return outcome;
    }
    Outcome::Unsat
}

fn recurse_dpll(knowledge_base: &KnowledgeBase, decision: Literal) -> Option<Outcome> {
    let new_knowledge_base = apply_decision(knowledge_base.clone(), decision);
    if let Outcome::Sat(mut assignment) = run_dpll(new_knowledge_base) {
        assignment.insert(decision.0, decision.1);
        return Some(Outcome::Sat(assignment));
    }
    None
}

fn get_decision(knowledge_base: &KnowledgeBase) -> Literal {
    *knowledge_base.iter().next().unwrap().iter().next().unwrap()
}

fn apply_decision(mut knowledge_base: KnowledgeBase, decision: Literal) -> KnowledgeBase {
    let negated_decision = &negate_literal(decision);
    knowledge_base.retain(|c| !c.contains(&decision));

    for clause in &mut knowledge_base {
        clause.retain(|l| l != negated_decision);
    }

    knowledge_base
}

fn simplify(mut knowledge_base: KnowledgeBase) -> KnowledgeBase {
    loop {
        let decisions: Vec<_> = knowledge_base
            .iter()
            .filter(|c| c.len() == 1)
            .map(|c| *c.iter().next().unwrap())
            .collect();

        if decisions.is_empty() {
            break;
        }

        for decision in decisions {
            knowledge_base = apply_decision(knowledge_base, decision);
        }
    }

    knowledge_base
}
