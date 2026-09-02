use std::hash::BuildHasher;
use std::{collections::HashSet, hash::Hash};
use std::collections::HashMap;
use std::fmt::Debug;

use rapidhash::fast::{RandomState};
use serde::{Deserialize, Serialize};

use crate::codegen::intermediate::element::ElementIR;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[
    serde(
        bound(
            serialize = "K: Serialize, V: Serialize",
            deserialize = "K: Deserialize<'de>, V: Deserialize<'de>"
        )
    )
]
pub struct BiMap<K: Eq + Hash, V: Eq + Hash, S: BuildHasher + Default = RandomState> {
    map_direct: HashMap<K, V, S>,
    map_inverse: HashMap<V, K, S>
}

impl<K: Eq + Hash + Clone, V: Eq + Hash + Clone, S: BuildHasher + Default> BiMap<K, V, S> {
    pub fn new() -> BiMap<K, V, S> {
        BiMap { map_direct: HashMap::default(), map_inverse: HashMap::default() }
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
pub struct HashSetMap<K: Hash + Eq, V: Hash + Eq, S: BuildHasher + Default = RandomState> {
    map: HashMap<K, HashSet<V, S>, S>
}

impl<K: Debug + Hash + Eq, V: Debug + Hash + Eq, S: BuildHasher + Default> Debug for HashSetMap<K, V, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.map)
    }
}

impl<K: Hash + Eq + Clone, V: Hash + Eq, S: BuildHasher + Default> HashSetMap<K, V, S> {
    pub fn new() -> HashSetMap<K, V, S> {
        HashSetMap { map: HashMap::default() }
    }

    pub fn get(&self, key: &K) -> Option<&HashSet<V, S>> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, item: V) {
        match self.map.get_mut(&key) {
            Some(s) => {
                s.insert(item);
            },

            None => {
                let mut set = HashSet::default();
                set.insert(item);
                self.map.insert(key, set);
            }
        }
    }

    pub fn remove(&mut self, key: K) -> Option<HashSet<V, S>> {
        self.map.remove(&key)
    }

    pub fn extend<I: IntoIterator<Item = V>>(&mut self, key: K, values: I) -> Option<&HashSet<V, S>> {
        for item in values.into_iter() {
            self.insert(key.clone(), item);
        }

        self.get(&key)
    }

    pub fn union_skipping(self, skip: K) -> HashSet<V, RandomState> {
        let mut set = HashSet::default();
        for (key, value) in self {
            if key == skip {
                continue;
            }
            
            set.extend(value);
        };

        set
    }
    
    pub fn remove_all_keys_matching(&mut self, f: impl FnMut(&K, &mut HashSet<V, S>) -> bool) -> impl Iterator<Item = (K, HashSet<V, S>)> {
        self.map.extract_if(f)
    }
}

impl<K: Hash + Eq + Clone, V: Hash + Eq, S: BuildHasher + Default> IntoIterator for HashSetMap<K, V, S> {
    type Item = (K, HashSet<V, S>);
    type IntoIter = <HashMap<K, HashSet<V, S>, S> as IntoIterator>::IntoIter;
    
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}