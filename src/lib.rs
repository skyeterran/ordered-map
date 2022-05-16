//! A [`HashVec`] is a hash map / dictionary whose key-value pairs are stored (and can be iterated over) in a fixed order, by default the order in which they were inserted into the hashvec. It's essentially a vector whose values can be inserted/retrieved with keys.
//! # Example
//! ```
//! use hashvec::HashVec;
//! 
//! // Create a new hashvec containing pairs of animal names and species
//! let mut hashvec: HashVec<&'static str, &'static str> = HashVec::new();
//! 
//! // Insert values into the hashvec (HashMap-style)
//! // Inserting overwrites existing keys' entries in-place
//! hashvec.insert("Doug", "Kobold");
//! hashvec.insert("Skye", "Hyena");
//! hashvec.insert("Lee", "Shiba");
//! hashvec.insert("Sock", "Man");
//! 
//! // Push values onto the hashvec (Vector-style)
//! // Pushing overwrites existing keys' entries and moves them to the end
//! hashvec.push(("Salad", "Wolf"));
//! hashvec.push(("Finn", "Human"));
//! hashvec.push(("Jake", "Dog"));
//! hashvec.push(("Susie", "Squid"));
//! 
//! // Access a value by key
//! match hashvec.get(&"Finn") {
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
//! let lee_index = hashvec.index(&"Lee").unwrap();
//! assert_eq!(lee_index, 2);
//! 
//! // Get the length of the hashvec
//! let hashvec_length = hashvec.len();
//! assert_eq!(hashvec_length, 8);
//! 
//! // Change an entry's key in-place
//! hashvec.rename(&"Salad", "Caesar");
//! assert_eq!(hashvec[4], ("Caesar", "Dog"));
//! 
//! // Mutate a value
//! match hashvec.get_mut(&"Sock") {
//!     Some(value) => {
//!         *value = "Guinea Pig";
//!     },
//!     None => {}
//! }
//! assert_eq!(*hashvec.get(&"Sock").unwrap(), "Guinea Pig");
//! 
//! // Remove an entry
//! hashvec.remove(&"Doug");
//! assert_eq!(hashvec.get(&"Doug"), None);
//! 
//! // Swap the locations of two entries by their keys
//! hashvec.swap_keys(&"Lee", &"Skye");
//! assert_eq!(hashvec.index(&"Lee").unwrap(), 0);
//! assert_eq!(hashvec.index(&"Skye").unwrap(), 1);
//! 
//! // Now swap them again, by their indices
//! hashvec.swap_indices(0, 1);
//! assert_eq!(hashvec[0], ("Skye", "Hyena"));
//! assert_eq!(hashvec[1], ("Lee", "Shiba"));
//! 
//! // Iterate over each of the key-value pairs in the hashvec
//! for (k, v) in hashvec.into_iter() {
//!     println!("{} is a {}!", k, v);
//! }
//! 
//! // Remove an entry from the end of the hashvec
//! let last_entry = hashvec.pop();
//! assert_eq!(last_entry.unwrap(), ("Susie", "Squid"));
//! 
//! // Clear the hashvec
//! hashvec.clear();
//! ```

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use core::ops::Index;

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

    /// Returns the number of elements in the hashvec.
    pub fn len(&self) -> usize {
        self.entries.len()
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

    /// Inserts an entry into the hashvec, or replaces an existing one.
    pub fn insert(&mut self, k: K, v: V) {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => {
                // If the key was already in the hashvec, update its entry in-place
                self.entries[*index].1 = v;
            },
            None => {
                // If the entry wasn't in the hashvec already, add it
                self.order.insert(calculate_hash(&k), self.entries.len());
                self.entries.push((k, v));
            }
        }
    }

    /// Appends an entry to the back of the hashvec.
    /// 
    /// If an entry with an identical key was already in the hashvec, it is removed before the new entry is inserted.
    /// 
    /// # Panics
    /// Panics if the new capacity either overflows `usize` or exceeds `isize::MAX` bytes.
    pub fn push(&mut self, entry: (K, V)) {
        if self.contains_key(&entry.0) {
            self.remove(&entry.0);
        }

        let key_hash = calculate_hash(&entry.0);
        self.order.insert(key_hash, self.entries.len());
        self.entries.push(entry);
    }

    /// Removes the last entry from the hashvec and returns it (or `None` if the hashvec is empty).
    pub fn pop(&mut self) -> Option<(K, V)> {
        let last_entry = self.entries.pop();

        match last_entry {
            Some(entry) => {
                let key_hash = calculate_hash(&entry.0);

                // Stop tracking the popped entry's key
                self.order.remove(&key_hash);

                Some(entry)
            },
            None => None
        }
    }

    /// Swaps the location of the provided keys' entries
    /// 
    /// If either one of the keys is not already in the hashvec, this is a no-op.
    pub fn swap_keys(&mut self, key_a: &K, key_b: &K) {
        let key_hash_a = calculate_hash(&key_a);
        let key_hash_b = calculate_hash(&key_b);
        let op_valid = self.order.contains_key(&key_hash_a) && self.order.contains_key(&key_hash_b);

        if op_valid {
            // Swap the tracked order
            let old_index_a = *self.order.get(&key_hash_a).unwrap();
            let old_index_b = *self.order.get(&key_hash_b).unwrap();
            self.order.insert(key_hash_a, old_index_b);
            self.order.insert(key_hash_b, old_index_a);

            // Swap the actual entries
            self.entries.swap(old_index_a, old_index_b);
        }
    }

    /// Swaps the location of the entries at the provided indices
    /// 
    /// If either one of the indices exceeds the current length of the hashvec, this is a no-op.
    pub fn swap_indices(&mut self, index_a: usize, index_b: usize) {
        if index_a.max(index_b) < self.len() {
            let key_hash_a = calculate_hash(&self.entries[index_a].0);
            let key_hash_b = calculate_hash(&self.entries[index_b].0);
    
            // Swap the tracked order
            let old_index_a = *self.order.get(&key_hash_a).unwrap();
            let old_index_b = *self.order.get(&key_hash_b).unwrap();
            self.order.insert(key_hash_a, old_index_b);
            self.order.insert(key_hash_b, old_index_a);

            // Swap the actual entries
            self.entries.swap(old_index_a, old_index_b);
        }
    }

    /// Returns `true` if the hashvec contains an entry corresponding to the provided key.
    pub fn contains_key(&self, k: &K) -> bool {
        self.order.contains_key(&calculate_hash(k))
    }

    /// Returns a reference to the value corresponding to the key, if it exists.
    pub fn get(&self, k: &K) -> Option<&V> {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => Some(&self.entries[*index].1),
            None => None
        }
    }

    /// Returns a mutable reference to the value corresponding to the key, if it exists.
    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        match self.order.get(&calculate_hash(&k)) {
            Some(index) => Some(&mut self.entries[*index].1),
            None => None
        }
    }

    /// Changes an entry's key, preserving and returning a reference to the associated value.
    /// 
    /// If the hashvec did not have an entry corresponding to the old key, `None` is returned.
    pub fn rename(&mut self, old_key: &K, new_key: K) -> Option<&V> {
        let old_key_hash = calculate_hash(old_key);

        let index_opt = match self.order.get(&old_key_hash) {
            Some(index) => Some(*index),
            None => None
        };

        match index_opt {
            Some(index) => {
                let new_key_hash = calculate_hash(&new_key);

                // Change the entry's key
                self.entries[index].0 = new_key;

                // Stop tracking the old key hash
                self.order.remove(&old_key_hash);

                // Start tracking the new key hash
                self.order.insert(new_key_hash, index);

                // Return the corresponding value
                Some(&self.entries[index].1)
            },
            None => None
        }
    }

    /// Removes a key from the hashvec, returning the stored key and value if the key was previously in the hashvec.
    pub fn remove_entry(&mut self, k: &K) -> Option<(K, V)> {
        let key_hash = calculate_hash(k);
        
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
    pub fn index(&self, k: &K) -> Option<usize> {
        match self.order.get(&calculate_hash(k)) {
            Some(index) => Some(*index),
            None => None
        }
    }

    /// Removes a key from the hashvec, returning the stored value if the key was previously in the hashvec.
    pub fn remove(&mut self, k: &K) -> Option<V> {
        match self.remove_entry(k) {
            Some((_, v)) => Some(v),
            None => None
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the `HashVec`. The collection may reserve more space to avoid frequent reallocations.
    /// 
    /// # Panics
    /// Panics if the new capacity either overflows `usize` or exceeds `isize::MAX` bytes.
    pub fn reserve(&mut self, additional: usize) {
        self.entries.reserve(additional);
        self.order.reserve(additional);
    }

    /// Shrinks the capacity of the hashvec with a lower limit.
    /// 
    /// The capacity will remain at least as large as both the length and the supplied value.
    /// 
    /// If the current capacity is less than the lower limit, this is a no-op.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.entries.shrink_to(min_capacity);
        self.order.shrink_to(min_capacity);
    }

    /// Shrinks the capacity of the hashvec as much as possible, according to internal rules.
    pub fn shrink_to_fit(&mut self) {
        self.entries.shrink_to_fit();
        self.order.shrink_to_fit();
    }
}

impl<K: Eq + Hash, V> Index<usize> for HashVec<K, V> {
    type Output = (K, V);
    fn index(&self, i: usize) -> &(K, V) {
        &self.entries[i]
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