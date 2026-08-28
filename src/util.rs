use std::{collections::HashSet, hash::Hash};
use std::collections::HashMap;
use std::fmt::Debug;

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


#[derive(Clone)]
pub struct HashSetMap<K: Hash + Eq, V: Hash + Eq> {
    map: HashMap<K, HashSet<V>>
}

impl<K: Debug + Hash + Eq, V: Debug + Hash + Eq> Debug for HashSetMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.map)
    }
}

impl<K: Hash + Eq + Clone, V: Hash + Eq> HashSetMap<K, V> {
    pub fn new() -> HashSetMap<K, V> {
        HashSetMap { map: HashMap::new() }
    }

    pub fn get(&self, key: &K) -> Option<&HashSet<V>> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, item: V) {
        match self.map.get_mut(&key) {
            Some(s) => {
                s.insert(item);
            },

            None => {
                let mut set = HashSet::new();
                set.insert(item);
                self.map.insert(key, set);
            }
        }
    }

    pub fn extend<I: IntoIterator<Item = V>>(&mut self, key: K, values: I) -> Option<&HashSet<V>> {
        for item in values.into_iter() {
            self.insert(key.clone(), item);
        }

        self.get(&key)
    }

    pub fn union_skipping(self, skip: K) -> HashSet<V> {
        let mut set = HashSet::new();
        for (key, value) in self {
            if key == skip {
                continue;
            }
            
            set.extend(value);
        };

        set
    }
}

impl<K: Hash + Eq + Clone, V: Hash + Eq> IntoIterator for HashSetMap<K, V> {
    type Item = (K, HashSet<V>);
    type IntoIter = <HashMap<K, HashSet<V>> as IntoIterator>::IntoIter;
    
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
    
}