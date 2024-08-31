use std::sync::atomic::{AtomicU32, Ordering};

use crate::*;

pub static DECISION_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn run_dpll(knowledge_base: KnowledgeBase) -> Outcome {
    State {
        knowledge_base,
        assignment: Assignment::new(),
    }
    .dpll()
}

#[derive(Clone)]
pub struct State {
    pub knowledge_base: KnowledgeBase,
    pub assignment: Assignment,
}

impl State {
    pub fn simplify(&mut self) {
        loop {
            let decisions: Vec<_> = self
                .knowledge_base
                .iter()
                .filter(|c| c.len() == 1)
                .map(|c| *c.iter().next().unwrap())
                .collect();

            if decisions.is_empty() {
                break;
            }

            for &decision in &decisions {
                self.apply_decision(decision);
            }
        }
    }

    pub fn apply_decision(&mut self, decision: Literal) {
        self.assignment.insert(decision.0, decision.1);

        let negated_decision = &negate_literal(decision);
        self.knowledge_base.retain(|c| !c.contains(&decision));

        for clause in &mut self.knowledge_base {
            clause.retain(|l| l != negated_decision);
        }
    }

    fn dpll(mut self) -> Outcome {
        self.simplify();

        // check satisfied
        if self.knowledge_base.is_empty() {
            return Outcome::Sat(self.assignment);
        }
        // check unsatisfiable
        if self.knowledge_base.iter().any(|c| c.is_empty()) {
            return Outcome::Unsat;
        }

        let decision = get_decision(&self.knowledge_base);
        println!("{}", decision.draw());
        DECISION_COUNTER.fetch_add(1, Ordering::AcqRel);

        let mut clone = self.clone();
        clone.apply_decision(decision);
        let outcome1 = clone.dpll();
        if let Outcome::Sat(_) = outcome1 {
            return outcome1;
        }

        let negated_decision = negate_literal(decision);
        self.apply_decision(negated_decision);
        self.dpll()
    }
}
