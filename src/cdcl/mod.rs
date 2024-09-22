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
pub enum Cause {
    Branch,

    // Law of excluded middle: we tried the opposite and it failed.
    Lem,

    // We derived this literal by unit propagation of this clause.
    Unit(Clause)
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
            self.dump_stack();

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
            self.apply_decision(lit, Cause::Branch);
        }
    }

    fn dump_stack(&self) {
        for (var, (b, cause)) in self.cause_stack.iter() {
            println!("----------");
            println!("-STACK:-");
            println!("----------");
            let var = gsymb_get(*var);
            let lit: String = if *b {
                format!("{var}")
            } else {
                format!("~{var}")
            };
            let s: String = match cause {
                Cause::Lem => "lem".to_string(),
                Cause::Branch => "branch".to_string(),
                Cause::Unit(c) => format!("unit({:?})", c),
            };
            println!("{} - {}", lit, s);
            println!("----------");
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

    fn get(&self, v: Var) -> Option<bool> {
        self.cause_stack.get(&v).map(|(x, _)| *x)
    }
}
