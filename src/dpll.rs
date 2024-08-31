use crate::draw::Draw;
use crate::heuristics::get_decision;
use crate::{negate_literal, Literal, Outcome};
use crate::knowledge_base::{Assignment, KnowledgeBase};

pub fn run_dpll(knowledge_base: KnowledgeBase) -> Outcome {
    State {
        knowledge_base,
        assignment: Assignment::new(),
    }.dpll()
}

#[derive(Clone)]
struct State {
    knowledge_base: KnowledgeBase,
    assignment: Assignment,
}

impl State {
    fn simplify(&mut self) {
        loop {
            let decisions: Vec<_> = self.knowledge_base
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

    fn apply_decision(&mut self, decision: Literal) {
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

        let mut clone = self.clone();
        clone.apply_decision(decision);
        let outcome1 = clone.dpll();
        if let Outcome::Sat(_) = outcome1 { return outcome1; }

        let negated_decision = negate_literal(decision);
        self.apply_decision(negated_decision);
        self.dpll()
    }
}
