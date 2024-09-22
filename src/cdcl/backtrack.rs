use crate::*;

impl Cdcl {
    // returns whether we should continue. false means to return Outcome::Unsat.
    pub fn backtrack(&mut self, c: &Clause) -> bool {
        let mut open: Clause = c.clone();

        loop {
            let Some((v, (b, cause))) = self.cause_stack.pop() else { return false; };
            let lit = (v, b);
            let neg = negate_literal(lit);

            // only care for literals that are relevant for this contradiction.
            if open.contains(&neg) {
                match cause {
                    Cause::Branch => {
                        self.unsimplify();
                        self.apply_decision(neg, Cause::Lem);
                        self.satisfied.push(open);
                        return true;
                    },
                    Cause::Unit(clause) => {
                        open.extend(clause);
                        open.remove(&neg);
                    },
                    _ => {},
                }
            }
        }
    }

    // re-open clauses that were closed in a previous branch.
    fn unsimplify(&mut self) {
        for (x, y) in self.open.iter_mut() {
            for z in y.clone() {
                if self.cause_stack.get(&z.0).is_none() {
                    y.remove(&z);
                    x.insert(z);
                }
            }
        }

        // TODO make better.
        self.open.extend(self.satisfied.iter().map(|x| (x.clone(), Default::default())));
        self.satisfied = Default::default();
    }
}
