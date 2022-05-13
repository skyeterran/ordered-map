//! An [`HashVec`] is a hashvec / dictionary whose key-value pairs are stored (and can be iterated over) in a fixed order, by default the order in which they were inserted into the hashvec. It's essentially a vector whose values can be inserted/retrieved with keys.
//! # Example
//! ```
//! // Create a new hashvec containing pairs of animal names and species
//! let mut hashvec: HashVec<&'static str, &'static str> = HashVec::new();
//! 
//! // Insert values into the hashvec
//! hashvec.insert("Doug", "Kobold");
//! hashvec.insert("Skye", "Jaguar");
//! hashvec.insert("Lee", "Shiba");
//! hashvec.insert("Sock", "Man");
//! hashvec.insert("Salad", "Dog");
//! hashvec.insert("Finn", "Human");
//! hashvec.insert("Jake", "Dog");
//! 
//! // Access a value by key
//! match hashvec.get("Finn") {
//!     Some(value) => {
//!         assert_eq!(*value, "Human");
//!     },
//!     None => {}
//! }
//! 
//! // Access an entry by index
//! let lee_value = hashvec[2];
//! assert_eq!(lee_value, ("Lee", "Shiba"));
//! 
//! // Get the index of a key
//! let lee_index = hashvec.index("Lee").unwrap();
//! assert_eq!(lee_index, 2);
//! 
//! // Mutate a value
//! match hashvec.get_mut("Sock") {
//!     Some(value) => {
//!         *value = "Guinea Pig";
//!     },
//!     None => {}
//! }
//! assert_eq!(*hashvec.get("Sock").unwrap(), "Guinea Pig");
//! 
//! // Remove a value
//! hashvec.remove("Doug");
//! assert_eq!(hashvec.get("Doug"), None);
//! 
//! // Iterate over each of the key-value pairs in the hashvec
//! for (k, v) in hashvec.into_iter() {
//!     println!("{} is a {}!", k, v);
//! }
//! 
//! // Clear the hashvec
//! map.clear();
//! ```

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct HashVec<K: Eq + Hash, V> {
    entries: Vec<(K, V)>,
    order: HashMap<u64, usize>
}

impl<K: Eq + Hash, V> HashVec<K, V> {
    /// Creates a new, empty map.
    pub fn new() -> HashVec<K, V> {
        HashVec {
            entries: Vec::new(),
            order: HashMap::new()
        }
    }

    /// Creates a new, empty hashvec with the specified capacity.
    pub fn with_capacity(capacity: usize) -> HashVec<K, V> {
        HashVec {
            entries: Vec::with_capacity(capacity),
            order: HashMap::with_capacity(capacity)
        }
    }

    /// Returns the number of elements the hashvec can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.entries.capacity().min(self.order.capacity())
    }

    /// Returns `true` if the hashvec contains no elements.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clears the hashvec, removing all entries.
    /// 
    /// Keep in mind this will not reallocate memory.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }

    /// Inserts an entry into the hashvec, or replaces an existing one
    pub fn insert(&mut self, k: K, v: V) {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => {
                // If the entry was already in the hashvec, update it in-place
                self.entries[*index] = (k, v);
            },
            None => {
                // If the entry wasn't in the hashvec already, add it
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

    /// Removes a key from the hashvec, returning the stored key and value if the key was previously in the hashvec.
    pub fn remove_entry(&mut self, k: K) -> Option<(K, V)> {
        let key_hash = calculate_hash(&k);
        
        let index_opt = match self.order.get(&key_hash) {
            Some(index) => Some(*index),
            None => None
        };

        match index_opt {
            Some(index) => {
                // Get the entry and then remove it from the hashvec entirely before returning the value
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
    
    // Swaps the positions of entries `a` and `b` within the hashvec.
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

    /// Removes a key from the hashvec, returning the stored value if the key was previously in the hashvec.
    pub fn remove(&mut self, k: K) -> Option<V> {
        match self.remove_entry(k) {
            Some((_, v)) => Some(v),
            None => None
        }
    }
}

impl<K: Eq + Hash, V> Index<usize> for HashVec<K, V> {
    type Output = (K, V);
    fn index(&self, i: usize) -> &(K, V) {
        &self.entries[i]
    }
}

impl<K: Eq + Hash, V> IndexMut<usize> for HashVec<K, V> {
    fn index_mut(&mut self, i: usize) -> &mut (K, V) {
        &mut self.entries[i]
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a HashVec<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashVecIter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        HashVecIter {
            ordered_map: self,
            index: 0
        }
    }
}

// Wrapping iterator struct
pub struct HashVecIter<'a, K: Eq + Hash, V> {
    ordered_map: &'a HashVec<K, V>,
    index: usize
}

impl<'a, K: Eq + Hash, V> Iterator for HashVecIter<'a, K, V> {
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