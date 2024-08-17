use crate::*;

fn should(s: &str, outcome: Outcome) {
    let a = parse(s);
    assert_eq!(run(a), outcome);
}

fn should_sat(s: &str) {
    should(s, Outcome::Sat);
}

fn should_unsat(s: &str) {
    should(s, Outcome::Unsat);
}

#[test]
fn t1() {
    should_sat("(A)");
}

#[test]
fn t2() {
    should_sat("(A)&(B)&(~A|B)");
}

#[test]
fn t3() {
    should_unsat("(A)&(B)&(~A|~B)");
}

#[test]
fn t4() {
    should_sat("(A|B)&(~A|~B)&(A|~B)");
}

#[test]
fn t5() {
    should_unsat("(A|B)&(~A|~B)&(A|~B)&(~A|B)");
}
