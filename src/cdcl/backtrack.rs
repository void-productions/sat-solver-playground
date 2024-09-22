use crate::*;

impl Cdcl {
    // returns whether we should continue. false means to return Outcome::Unsat.
    pub fn backtrack(&mut self, c: &Clause) -> bool {
        let mut open: BTreeSet<_> = c.iter().map(|x| negate_literal(*x)).collect();

        loop {
            let Some((v, (b, cause))) = self.cause_stack.pop() else { return false; };

            // only care for literals that are relevant for this contradiction.
            if open.contains(&(v, b)) {
                match cause {
                    Cause::Branch => {
                        self.unsimplify();
                        self.apply_decision((v, !b), Cause::Lem);
                        self.satisfied.push(open);
                        return true;
                    },
                    Cause::Unit(clause) => {
                        todo!()
                    },
                    _ => {},
                }
            }
        }
    }

    // re-open clauses that were closed in a previous branch.
    fn unsimplify(&mut self) {
        todo!()
    }
}
