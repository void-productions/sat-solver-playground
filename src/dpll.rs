use crate::draw::Draw;
use crate::heuristics::get_decision;
use crate::{negate_literal, Assignment, KnowledgeBase, Literal, Outcome};

pub fn run_dpll(knowledge_base: KnowledgeBase) -> Outcome {
    let (knowledge_base, simplify_assignment) = simplify(knowledge_base);

    // check satisfied
    if knowledge_base.is_empty() {
        return Outcome::Sat(simplify_assignment);
    }
    // check unsatisfiable
    if knowledge_base.iter().any(|c| c.is_empty()) {
        return Outcome::Unsat;
    }

    let decision = get_decision(&knowledge_base);
    println!("{}", decision.draw());
    if let Outcome::Sat(mut assignment) = recurse_dpll(&knowledge_base, decision) {
        assignment.extend(simplify_assignment);
        return Outcome::Sat(assignment);
    }
    let negated_decision = negate_literal(decision);
    if let Outcome::Sat(mut assignment) = recurse_dpll(&knowledge_base, negated_decision) {
        assignment.extend(simplify_assignment);
        return Outcome::Sat(assignment);
    }
    Outcome::Unsat
}

fn recurse_dpll(knowledge_base: &KnowledgeBase, decision: Literal) -> Outcome {
    let new_knowledge_base = apply_decision(knowledge_base.clone(), decision);
    if let Outcome::Sat(mut assignment) = run_dpll(new_knowledge_base) {
        assignment.insert(decision.0, decision.1);
        return Outcome::Sat(assignment);
    }
    Outcome::Unsat
}

fn apply_decision(mut knowledge_base: KnowledgeBase, decision: Literal) -> KnowledgeBase {
    let negated_decision = &negate_literal(decision);
    knowledge_base.retain(|c| !c.contains(&decision));

    for clause in &mut knowledge_base {
        clause.retain(|l| l != negated_decision);
    }

    knowledge_base
}

fn simplify(mut knowledge_base: KnowledgeBase) -> (KnowledgeBase, Assignment) {
    let mut decisions_complete = Assignment::new();
    loop {
        let decisions: Vec<_> = knowledge_base
            .iter()
            .filter(|c| c.len() == 1)
            .map(|c| *c.iter().next().unwrap())
            .collect();

        if decisions.is_empty() {
            break;
        }

        for &decision in &decisions {
            knowledge_base = apply_decision(knowledge_base, decision);
        }
        decisions_complete.extend(decisions);
    }

    (knowledge_base, decisions_complete)
}
