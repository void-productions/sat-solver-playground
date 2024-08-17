use crate::*;

pub trait Draw {
    fn draw(&self, smap: &SymbolMap) -> String;
}

impl Draw for Id {
    fn draw(&self, smap: &SymbolMap) -> String {
        smap.get(*self).to_string()
    }
}

impl Draw for Literal {
    fn draw(&self, smap: &SymbolMap) -> String {
        let (var, b) = self;
        let opt_neg = if *b { "" } else { "~" };
        format!("{}{}", opt_neg, var.draw(smap))
    }
}

impl Draw for Clause {
    fn draw(&self, smap: &SymbolMap) -> String {
        let v: Vec<_> = self.iter().map(|x| x.draw(smap)).collect();
        format!("({})", v.join("|"))
    }
}

impl Draw for ClauseSet {
    fn draw(&self, smap: &SymbolMap) -> String {
        let v: Vec<_> = self.iter().map(|x| x.draw(smap)).collect();
        v.join("&")
    }
}
