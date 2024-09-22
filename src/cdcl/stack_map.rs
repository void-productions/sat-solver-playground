use crate::*;

// behaves like a stack, but also a map.
pub struct StackMap<K: Eq + Hash + Clone, V: Clone> {
    stack: Vec<(K, V)>,
    map: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> StackMap<K, V> {
    pub fn new() -> Self {
        Self {
            stack: Default::default(),
            map: HashMap::new(),
        }
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        let (k, v) = self.stack.pop()?;
        self.map.remove(&k).unwrap();
        Some((k, v))
    }

    pub fn push(&mut self, k: K, v: V) {
        self.stack.push((k.clone(), v.clone()));
        self.map.insert(k, v);
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.map.get(k)
    }

    pub fn iter(&self) -> impl Iterator<Item=&(K, V)> {
        self.stack.iter()
    }
}
