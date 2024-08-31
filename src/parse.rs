use crate::*;

// syntax: (X | ~X) & (Y)
pub fn parse(s: &str) -> KnowledgeBase {
    let mut s = s.to_string();
    loop {
        let n = s.len();
        s = s.replace("\n", "");
        s = s.replace(" ", "");
        if n == s.len() {
            break;
        }
    }
    let mut s = &*s;

    let mut out = KnowledgeBase::new();
    loop {
        if s.len() == 0 {
            break;
        }
        let (c, s2) = parse_clause(&s);
        out.push(c);
        if s2.len() == 0 {
            break;
        }
        assert!(s2.chars().next().unwrap() == '&');
        s = &s2[1..];
    }
    out
}

fn parse_clause<'s>(s: &'s str) -> (Clause, &'s str) {
    assert!(s.chars().next().unwrap() == '(');
    let mut s = &s[1..];

    let mut out = Clause::default();

    loop {
        let ch = s.chars().next().unwrap();
        if ch == ')' {
            return (out, &s[1..]);
        }
        let (lit, s2) = parse_literal(s);
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

fn parse_literal<'s>(mut s: &'s str) -> (Literal, &'s str) {
    let ch = s.chars().next().unwrap();
    let mut b = true;
    if ch == '~' {
        s = &s[1..];
        b = false;
    }
    let (id, s) = parse_symbol(s);

    ((id, b), s)
}

fn alphanumeric(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || ('0'..='9').contains(&c) || c == '_'
}

fn parse_symbol<'s>(s: &'s str) -> (Id, &'s str) {
    let i = s.chars().position(|c| !alphanumeric(c)).unwrap_or(s.len());
    let substr = &s[0..i];
    let id = gsymb_add(substr.to_string());
    (id, &s[i..])
}
