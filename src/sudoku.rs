use crate::*;

// sudoku[x][y]
pub type Sudoku = Vec<Field>;

pub enum Field {
    Num(u8),
    Empty,
}

pub fn parse_sudoku(s: &str) -> Sudoku {
    let mut s = s.to_string();
    loop {
        let n = s.len();
        s = s.replace("\n", "");
        s = s.replace(" ", "");
        s = s.replace("\t", "");
        if n == s.len() { break; }
    }

    s.chars().map(|ch| {
        match ch {
            '0'..='9' => Field::Num(ch as u8 - '0' as u8),
            '-' => Field::Empty,
            _ => panic!(),
        }
    }).collect()
}

// exactly one of these variables should be true.
fn push_one_of(vars: Vec<Var>, c: &mut KnowledgeBase, exactly: bool) {
    let disj = vars.iter().map(|x| (*x, true)).collect();
    c.insert(disj);

    if exactly {
        for x in &vars {
            for y in &vars {
                if x != y {
                    let mut clause = Clause::new();
                    clause.insert((*x, false));
                    clause.insert((*y, false));
                    c.insert(clause);
                }
            }
        }
    }
}

fn index_sets() -> Vec<BTreeSet<(u8, u8)>> {
    let mut out = Vec::new();

    // rows:
    for x in 1..=9 {
        let set = (1..=9).map(|y| (x, y)).collect();
        out.push(set);
    }

    // columns:
    for y in 1..=9 {
        let set = (1..=9).map(|x| (x, y)).collect();
        out.push(set);
    }

    // boxes:
    for x_ in [1, 4, 7] {
        for y_ in [1, 4, 7] {
            let mut inner = BTreeSet::new();
            for x in 0..3 {
                for y in 0..3 {
                    inner.insert((x_+x, y_+y));
                }
            }
            out.push(inner);
        }
    }

    out
}

fn base_clauseset() -> KnowledgeBase {
    let mut c = KnowledgeBase::new();

    // each cell contains exactly one value.
    for x in 1..=9 {
        for y in 1..=9 {
            let vars = (1..=9).map(|n| var(x, y, n)).collect();
            push_one_of(vars, &mut c, true);
        }
    }

    // each index set contains exactly one n.
    for n in 1..=9 {
        for set in index_sets() {
            let vars = set.iter().map(|(x, y)| var(*x, *y, n)).collect();
            push_one_of(vars, &mut c, true);
        }
    }

    c
}

// x, y, val in [1, 9]
fn var(x: u8, y: u8, val: u8) -> Id {
    let name = format!("v{}{}{}", x, y, val);
    gsymb_add(name)
}

// x, y in [1, 9]
fn idx(x: u8, y: u8) -> usize {
    let x = x as usize - 1;
    let y = y as usize - 1;
    x + y*9
}

pub fn sudoku_to_knowledge_base(s: &Sudoku) -> KnowledgeBase {
    let mut a = base_clauseset();
    for x in 1..=9 {
        for y in 1..=9 {
            if let Field::Num(n) = s[idx(x, y)] {
                let lit = (var(x, y, n), true);
                let mut clause = Clause::default();
                clause.insert(lit);
                a.insert(clause);
            }
        }
    }
    a
}
