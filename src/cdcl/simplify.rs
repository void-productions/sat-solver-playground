use crate::*;

impl Cdcl {
    // optionally returns an unsatisfiable clause.
    pub fn simplify(&mut self) {
        loop {
            let v: Vec<_> = self.open.iter()
                                     .filter(|(x, _)| x.len() == 1)
                                     .cloned()
                                     .collect();

            let mut changed = false;
            for (x, y) in v {
                let x = *x.iter().next().unwrap();

                // if it already got a value, don't overwrite it.
                if self.get(x.0).is_none() {
                    changed = true;
                    let mut combined = y.clone();
                    combined.insert(x);
                    self.apply_decision(x, Cause::Unit(combined));
                }
            }
            if !changed { break; }
        }
    }

    // applies the decision both in the assignment, and the knowledge base.
    pub fn apply_decision(&mut self, lit: Literal, cause: Cause) {
        assert!(self.get(lit.0).is_none());
        self.cause_stack.push(lit.0, (lit.1, cause));

        let neg = negate_literal(lit);
        self.open.retain_mut(|(x, y)| {
            if x.contains(&lit) {
                let combined = &*x | &*y;
                self.satisfied.push(combined);
                return false;
            }

            if x.contains(&neg) {
                x.remove(&neg);
                y.insert(neg);
            }

            true
        });
    }
}
