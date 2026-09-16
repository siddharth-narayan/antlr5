use std::hash::BuildHasher;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::{collections::HashSet, hash::Hash};
use std::collections::HashMap;
use std::fmt::Debug;

use rapidhash::fast::{RandomState};

use crate::codegen::intermediate::element::ElementIR;
use crate::codegen::intermediate::rule::RuleIR;

#[derive(PartialEq, Eq, Clone)]
pub struct HashArc<T> {
    inner: std::sync::Arc<T>
}

impl<T> HashArc<T> {
    pub fn new(inner: T) -> HashArc<T> {
        HashArc { inner: std::sync::Arc::new(inner) }
    }
}

impl<T: Debug> Debug for HashArc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, f)
    }
}

impl<T> Hash for HashArc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::sync::Arc::as_ptr(&self.inner).hash(state);
    }
}

impl<T> Deref for HashArc<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
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


pub struct Arena<T> {
    // Unlike a Vec<T>, capacity is the length of the vector, made of MaybeUnininit::unintialized()
    capacity: usize,

    mask: Vec<bool>,
    rules: Vec<MaybeUninit<T>>
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena { capacity: 0, mask: Vec::new(), rules: Vec::new() }
    }

    pub fn reserve(&mut self, size: usize) {
        for _ in self.capacity..size {
            self.mask.push(false);
            self.rules.push(MaybeUninit::uninit());
        }

        self.capacity = size;
    }

    // Should be used to reserve a spot for another element
    // An element MUST be inserted to this index before .finalize() or .get() are called on this index 
    pub unsafe fn mask(&mut self, index: usize) {
        if let Some(mask_element) = self.mask.get_mut(index) {
            *mask_element = true;
        };
    }

    pub fn set(&mut self, index: usize, item: T) {
        if let Some(element) = self.rules.get_mut(index) {

            *element = MaybeUninit::new(item);
            
            unsafe { self.mask(index) };

        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.mask.get(index).copied().unwrap_or_default() {
            Some(unsafe { MaybeUninit::assume_init_ref(self.rules.get(index)?) })
        } else {
            None
        }
    }

    pub fn push(&mut self, item: T) -> usize {
        self.push_index(0, item)
    }

    pub fn push_index(&mut self, mut index: usize, item: T) -> usize {
        let position = self.push_index_landing_location(index);

        self.set(position, item);
        
        return position;
    }



    // Where would an element be put if we pushed at a specific index
    pub fn push_index_landing_location(&mut self, mut index: usize) -> usize {
        loop {
            match self.mask.get(index) {
                Some(true) => (),
                Some(false) => {
                    return index;
                },

                None => {
                    self.reserve(index + 1);
                    return index;
                }
            }

            index += 1;
        }
    }

    pub fn finalize(mut self) -> Vec<T> {
        let mut final_vec = Vec::new();

        let mut index = 0;
        while let Some(true) = self.mask.get(index) {
            let rule = std::mem::replace(&mut self.rules[index], MaybeUninit::uninit());
            
            final_vec.push(unsafe { rule.assume_init() } );
            index += 1;
        }
        
        final_vec
    }
}

pub fn capitalize(string: String) -> String {
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_titlecase().collect::<String>() + c.as_str(),
    }
}