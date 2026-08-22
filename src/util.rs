use std::hash::Hash;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiMap<K: Eq + Hash, V: Eq + Hash> {
    map_direct: HashMap<K, V>,
    map_inverse: HashMap<V, K>
}

impl<K: Eq + Hash + Clone, V: Eq + Hash + Clone> BiMap<K, V> {
    pub fn new() -> BiMap<K, V> {
        BiMap { map_direct: HashMap::new(), map_inverse: HashMap::new() }
    }

    pub fn len(&self) -> usize {
        self.map_direct.len()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let result  = self.map_direct.insert(key.clone(), value.clone());
        self.map_inverse.insert(value, key);

        result
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map_direct.get(key)
    }

    pub fn get_inverse(&self, value: &V) -> Option<&K> {
        self.map_inverse.get(value)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map_direct.contains_key(key)
    }
}