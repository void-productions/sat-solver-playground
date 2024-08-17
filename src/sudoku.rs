use crate::*;

// sudoku[x][y]
type Sudoku = Vec<Field>;

enum Field {
    Num(u8),
    Empty,
}

fn parse_sudoku(s: &str) -> Sudoku {
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

fn base_clauseset(smap: &mut SymbolMap) -> ClauseSet {
    // for each (x, y)-cell:
    // - a big disjunction saying it should have at least one value
    // - and pairwise negated disjunctions to express it shouldn't have two

    // for each value n, and each cell set S:
    // - a big disjunction saying S should have at least one n
    // - and pairwise negated disjunctions to express it shouldn't have two

    todo!()
}

// x, y, val in [1, 9]
fn var(x: u8, y: u8, val: u8, smap: &mut SymbolMap) -> Id {
    let name = format!("v{}{}{}", x, y, val);
    smap.add(name)
}

// x, y in [1, 9]
fn idx(x: u8, y: u8) -> usize {
    let x = x as usize - 1;
    let y = y as usize - 1;
    x + y*9
}

fn sudoku_to_clauseset(s: &Sudoku, smap: &mut SymbolMap) -> ClauseSet {
    let mut a = base_clauseset(smap);
    for x in 1..=9 {
        for y in 1..=9 {
            if let Field::Num(n) = s[idx(x, y)] {
                let lit = (var(x, y, n, smap), true);
                let mut clause = Clause::default();
                clause.insert(lit);
                a.insert(clause);
            }
        }
    }
    a
}
