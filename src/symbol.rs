use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Id(usize);

pub struct SymbolMap {
    string_to_id: BTreeMap<String, Id>,
    id_to_string: Vec<String>,
}

impl SymbolMap {
    pub fn new() -> Self {
        Self {
            string_to_id: Default::default(),
            id_to_string: Default::default(),
        }
    }

    pub fn add(&mut self, x: String) -> Id {
        if let Some(y) = self.string_to_id.get(&x) {
            return *y;
        } else {
            let i = self.string_to_id.len();
            self.string_to_id.insert(x.clone(), Id(i));
            self.id_to_string.push(x);
            Id(i)
        }
    }

    pub fn get(&self, id: Id) -> &str {
        self.id_to_string.get(id.0).unwrap()
    }
}
