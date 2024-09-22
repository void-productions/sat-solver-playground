use crate::*;

mod stack_map;
pub use stack_map::*;

mod simplify;
pub use simplify::*;

mod backtrack;
pub use backtrack::*;

pub fn run_cdcl(k: KnowledgeBase) -> Outcome {
    let open = k.into_iter().map(|x| (x, BTreeSet::new())).collect();
    Cdcl {
        open,
        satisfied: Vec::new(),
        cause_stack: StackMap::new(),
    }.cdcl()
}

#[derive(Clone)]
enum Cause {
    Branch,

    // Law of excluded middle: we tried the opposite and it failed.
    Lem,

    // contains the assumed Literals, which made this clause a unit clause.
    // The negations of these literals were contained in the original clause.
    UnitClause(Vec<Literal>)
}

pub struct Cdcl {
    open: Vec<(Clause, Clause)>,
    satisfied: Vec<Clause>,
    cause_stack: StackMap<Var, (bool, Cause)>,
}

impl Cdcl {
    fn cdcl(mut self) -> Outcome {
        loop {
            self.simplify();

            // check satisfied:
            if self.open.is_empty() {
                return Outcome::Sat(self.current_assignment());
            }

            // check unsatisfied:
            if let Some((_, clause)) = self.open.iter().find(|(l, _)| l.is_empty()) {
                if !self.backtrack(&clause.clone()) {
                    return Outcome::Unsat;
                }
                continue;
            }

            let lit = self.get_decision();
            self.cause_stack.push(lit.0, (lit.1, Cause::Branch));
        }
    }

    fn current_assignment(&self) -> Assignment {
        self.cause_stack.iter()
                         .map(|(v, (b, _))| (*v, *b))
                         .collect()
    }

    fn get_decision(&self) -> Literal {
        let clause = &self.open.iter().next().unwrap().0;
        let out = *clause.iter().next().unwrap();
        out
    }
}
