use crate::*;

impl Cdcl {
    pub fn simplify(&mut self) {
        todo!()
    }

    // applies the decision both in the assignment, and the knowledge base.
    pub fn apply_decision(&mut self, lit: Literal, cause: Cause) {
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
            }

            true
        });
    }
}
