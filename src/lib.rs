use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct OrderedMap<K: Eq + Hash, V> {
    entries: Vec<(K, V)>,
    order: HashMap<u64, usize>
}

impl<K: Eq + Hash, V> OrderedMap<K, V> {
    pub fn new() -> OrderedMap<K, V> {
        OrderedMap {
            entries: Vec::new(),
            order: HashMap::new()
        }
    }

    /// Inserts an entry into the map, or replaces an existing one
    pub fn insert(&mut self, k: K, v: V) {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => {
                // If the entry was already in the map, update it in-place
                self.entries[*index] = (k, v);
            },
            None => {
                // If the entry wasn't in the map already, add it
                self.order.insert(calculate_hash(&k), self.entries.len());
                self.entries.push((k, v));
            }
        }
    }

    /// Gets the value associated with the provided key, if it exists
    pub fn get(&self, k: K) -> Option<&V> {
        match self.order.get(&calculate_hash(&k)) {
            Some(entry_index) => Some(&self.entries[*entry_index].1),
            None => None
        }
    }
}

impl<K: Eq + Hash, V> Index<usize> for OrderedMap<K, V> {
    type Output = V;
    fn index(&self, i: usize) -> &V {
        &self.entries[i].1
    }
}

impl<K: Eq + Hash, V> IndexMut<usize> for OrderedMap<K, V> {
    fn index_mut(&mut self, i: usize) -> &mut V {
        &mut self.entries[i].1
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a OrderedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = OrderedMapIter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        OrderedMapIter {
            ordered_map: self,
            index: 0
        }
    }
}

// Wrapping iterator struct
pub struct OrderedMapIter<'a, K: Eq + Hash, V> {
    ordered_map: &'a OrderedMap<K, V>,
    index: usize
}

impl<'a, K: Eq + Hash, V> Iterator for OrderedMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.ordered_map.entries.get(self.index) {
            Some((k, v)) => Some((k, v)),
            None => None
        };
        self.index += 1;
        result
    }
}

fn calculate_hash<K: Hash>(k: &K)-> u64 {
    let mut hasher = DefaultHasher::new();
    k.hash(&mut hasher);
    hasher.finish()
}