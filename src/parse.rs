use crate::*;

// syntax: (X | ~X) & (Y)
pub fn parse(s: &str, smap: &mut SymbolMap) -> ClauseSet {
    let mut s = s.to_string();
    loop {
        let n = s.len();
        s = s.replace("\n", "");
        s = s.replace(" ", "");
        if n == s.len() { break; }
    }
    let mut s = &*s;

    let mut out = BTreeSet::new();
    loop {
        if s.len() == 0 { break; }
        let (c, s2) = parse_clause(&s, smap);
        out.insert(c);
        if s2.len() == 0 { break; }
        assert!(s2.chars().next().unwrap() == '&');
        s = &s2[1..];
    }
    out
    
}

fn parse_clause<'s>(s: &'s str, smap: &mut SymbolMap) -> (Clause, &'s str) {
    assert!(s.chars().next().unwrap() == '(');
    let mut s = &s[1..];

    let mut out = Clause::default();

    loop {
        let ch = s.chars().next().unwrap();
        if ch == ')' {
            return (out, &s[1..]);
        }
        let (lit, s2) = parse_literal(s, smap);
        out.insert(lit);
        s = s2;

        let ch = s.chars().next().unwrap();
        if ch == ')' {
            return (out, &s[1..]);
        } else {
            assert!(ch == '|');
            s = &s[1..];
        }
    }
}

fn parse_literal<'s>(mut s: &'s str, smap: &mut SymbolMap) -> (Literal, &'s str) {
    let ch = s.chars().next().unwrap();
    let mut b = true;
    if ch == '~' {
        s = &s[1..];
        b = false;
    }
    let (id, s) = parse_symbol(s, smap);

    ((id, b), s)
}

fn alphanumeric(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || ('0'..='9').contains(&c) || c == '_'
}

fn parse_symbol<'s>(s: &'s str, smap: &mut SymbolMap) -> (Id, &'s str) {
    let i = s.chars().position(|c| !alphanumeric(c)).unwrap_or(s.len());
    let substr = &s[0..i];
    let id = smap.add(substr.to_string());
    (id, &s[i..])
}
