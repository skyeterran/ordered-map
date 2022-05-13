//! An [`OrderedMap`] is a hash map / dictionary whose key-value pairs are stored (and can be iterated over) in a fixed order, by default the order in which they were inserted into the map. It's essentially a vector whose values can be inserted/retrieved with keys.
//! # Example
//! ```
//! // Create a new map containing pairs of animal names and species
//! let mut map: OrderedMap<&'static str, &'static str> = OrderedMap::new();
//! 
//! // Insert values into the map
//! map.insert("Doug", "Kobold");
//! map.insert("Skye", "Jaguar");
//! map.insert("Lee", "Shiba");
//! map.insert("Sock", "Man");
//! map.insert("Salad", "Dog");
//! map.insert("Finn", "Human");
//! map.insert("Jake", "Dog");
//! 
//! // Access a value by key
//! match map.get("Finn") {
//!     Some(value) => {
//!         assert_eq!(*value, "Human");
//!     },
//!     None => {}
//! }
//! 
//! // Access an entry by index
//! let lee_value = map[2];
//! assert_eq!(lee_value, ("Lee", "Shiba"));
//! 
//! // Get the index of a key
//! let lee_index = map.index("Lee").unwrap();
//! assert_eq!(lee_index, 2);
//! 
//! // Mutate a value
//! match map.get_mut("Sock") {
//!     Some(value) => {
//!         *value = "Guinea Pig";
//!     },
//!     None => {}
//! }
//! assert_eq!(*map.get("Sock").unwrap(), "Guinea Pig");
//! 
//! // Remove a value
//! map.remove("Doug");
//! assert_eq!(map.get("Doug"), None);
//! 
//! // Iterate over each of the key-value pairs in the map
//! for (k, v) in map.into_iter() {
//!     println!("{} is a {}!", k, v);
//! }
//! 
//! // Clear the map
//! map.clear();
//! ```

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
    /// Creates a new, empty map.
    pub fn new() -> OrderedMap<K, V> {
        OrderedMap {
            entries: Vec::new(),
            order: HashMap::new()
        }
    }

    /// Creates a new, empty map with the specified capacity.
    pub fn with_capacity(capacity: usize) -> OrderedMap<K, V> {
        OrderedMap {
            entries: Vec::with_capacity(capacity),
            order: HashMap::with_capacity(capacity)
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.entries.capacity().min(self.order.capacity())
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clears the map, removing all entries.
    /// 
    /// Keep in mind this will not reallocate memory.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
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

    /// Returns a reference to the value corresponding to the key, if it exists.
    pub fn get(&self, k: K) -> Option<&V> {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => Some(&self.entries[*index].1),
            None => None
        }
    }

    /// Returns a mutable reference to the value corresponding to the key, if it exists.
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => Some(&mut self.entries[*index].1),
            None => None
        }
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    pub fn remove_entry(&mut self, k: K) -> Option<(K, V)> {
        let key_hash = calculate_hash(&k);
        
        let index_opt = match self.order.get(&key_hash) {
            Some(index) => Some(*index),
            None => None
        };

        match index_opt {
            Some(index) => {
                // Get the entry and then remove it from the map entirely before returning the value
                let value = self.entries.remove(index);
                
                // Remove the corresponding entry from the order hashmap
                self.order.remove(&key_hash);

                // Update the index on all the remaining entries which followed the one we just removed
                for (i, (k, v)) in self.entries.iter().enumerate() {
                    if i >= index {
                        self.order.insert(calculate_hash(&self.entries[i].0), i);
                    }
                }

                // Now return the value we retained earlier
                Some(value)
            },
            None => None
        }
    }
    
    // Swaps the positions of entries `a` and `b` within the map.
    //pub fn swap(&mut self, a: K, b: K) {
        //
    //}

    /// Returns the index of the provided key, if the key exists.
    pub fn index(&self, k: K) -> Option<usize> {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => Some(*index),
            None => None
        }
    }

    /// Removes a key from the map, returning the stored value if the key was previously in the map.
    pub fn remove(&mut self, k: K) -> Option<V> {
        match self.remove_entry(k) {
            Some((_, v)) => Some(v),
            None => None
        }
    }
}

impl<K: Eq + Hash, V> Index<usize> for OrderedMap<K, V> {
    type Output = (K, V);
    fn index(&self, i: usize) -> &(K, V) {
        &self.entries[i]
    }
}

impl<K: Eq + Hash, V> IndexMut<usize> for OrderedMap<K, V> {
    fn index_mut(&mut self, i: usize) -> &mut (K, V) {
        &mut self.entries[i]
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