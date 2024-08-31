use crate::*;

// sudoku[x][y]
pub type Sudoku = Vec<Field>;

#[derive(Copy, Clone)]
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
        if n == s.len() {
            break;
        }
    }

    s.chars()
        .map(|ch| match ch {
            '0'..='9' => Field::Num(ch as u8 - '0' as u8),
            '-' => Field::Empty,
            _ => panic!(),
        })
        .collect()
}

pub fn print_sudoku(s: &Sudoku) {
    for chunk in s.chunks(9) {
        for f in chunk {
            match f {
                Field::Num(n) => {
                    print!("{n}");
                }
                Field::Empty => {
                    print!("-")
                }
            }
        }
        println!("");
    }
}

// exactly one of these variables should be true.
fn push_one_of(vars: Vec<Var>, c: &mut KnowledgeBase, exactly: bool) {
    let disj = vars.iter().map(|x| (*x, true)).collect();
    c.push(disj);

    if exactly {
        for x in &vars {
            for y in &vars {
                if x != y {
                    let mut clause = Clause::new();
                    clause.insert((*x, false));
                    clause.insert((*y, false));
                    c.push(clause);
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
                    inner.insert((x_ + x, y_ + y));
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
    x + y * 9
}

pub fn sudoku_to_knowledge_base(s: &Sudoku) -> KnowledgeBase {
    let mut a = base_clauseset();
    for x in 1..=9 {
        for y in 1..=9 {
            if let Field::Num(n) = s[idx(x, y)] {
                let lit = (var(x, y, n), true);
                let mut clause = Clause::default();
                clause.insert(lit);
                a.push(clause);
            }
        }
    }
    a
}

pub fn assigment_to_sudoku(assigment: &Assignment) -> Sudoku {
    let mut fields = vec![Field::Empty; 81];

    for (&var, &b) in assigment.iter().filter(|v| *v.1) {
        let s = (var, b).draw();
        let (x, y, n) = parse_variable(&s);
        fields[x - 1 + (y - 1) * 9] = Field::Num(n as u8);
    }
    fields
}

fn parse_variable(input: &str) -> (usize, usize, usize) {
    // Remove the leading character and collect the remaining characters
    let digits: Vec<usize> = input[1..]
        .chars() // Convert string slice to an iterator of characters
        .map(|c| c.to_digit(10).expect("Expected a digit") as usize) // Convert each character to a digit
        .collect(); // Collect the results into a vector

    // Ensure the vector contains exactly three elements and return them as a tuple
    (digits[0], digits[1], digits[2])
}

pub fn get_example() -> &'static str {
    let _easy = "
-4- --- --5
568 -1- 4--
1-7 -54 -6-

--- --8 ---
73- 162 -98
685 47- -3-

4-- 8-- 25-
--- 7-6 3--
-26 3-5 --1
    ";

    let hard = "
8-- --- ---
--3 6-- ---
-7- -9- 2--

-5- --7 ---
--- -45 7--
--- 1-- -3-

--1 --- -68
--8 5-- -1-
-9- --- 4--
    ";

    let _empty = "
--- --- ---
--- --- ---
--- --- ---

--- --- ---
--- --- ---
--- --- ---

--- --- ---
--- --- ---
--- --- ---
    ";
    hard
}
