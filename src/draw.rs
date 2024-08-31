use crate::*;
use crate::knowledge_base::{Clause, KnowledgeBase};

pub trait Draw {
    fn draw(&self) -> String;
}

impl Draw for Id {
    fn draw(&self) -> String {
        gsymb_get(*self).to_string()
    }
}

impl Draw for Literal {
    fn draw(&self) -> String {
        let (var, b) = self;
        let opt_neg = if *b { "" } else { "~" };
        format!("{}{}", opt_neg, var.draw())
    }
}

impl Draw for Clause {
    fn draw(&self) -> String {
        let v: Vec<_> = self.iter().map(|x| x.draw()).collect();
        format!("({})", v.join("|"))
    }
}

impl Draw for KnowledgeBase {
    fn draw(&self) -> String {
        let v: Vec<_> = self.iter().map(|x| x.draw()).collect();
        v.join("&")
    }
}
