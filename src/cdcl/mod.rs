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

    #[track_caller]
    pub fn inv(&self) {
        for c in &self.satisfied {
            assert!(self.sat_clause(&c));
        }

        for (x, y) in &self.open {
            let combined = x | y;
            assert!(!self.sat_clause(&combined));

            for z in x { assert!(self.get(z.0).is_none()); }
            for z in y { assert!(self.get(z.0) == Some(!z.1)); }
        }
    }

    fn all_clauses(&self) -> Vec<Clause> {
        let mut out: Vec<Clause> = Vec::new();
        out.extend(self.satisfied.iter().cloned());

        for (x, y) in &self.open {
            out.push(x | y);
        }

        out
    }

    fn add_clause(&mut self, c: Clause) {
        if self.sat_clause(&c) {
            self.satisfied.push(c);
        } else {
            let rest = c.iter().filter(|lit| self.sat_lit(negate_literal(**lit))).cloned().collect();
            let open = &c - &rest;
            self.open.push((open, rest));
        }
    }

    fn sat_lit(&self, lit: Literal) -> bool {
        self.get(lit.0) == Some(lit.1)
    }

    fn sat_clause(&self, c: &Clause) -> bool {
        c.iter().any(|lit| self.sat_lit(*lit))
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
