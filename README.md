# hashvec
A HashVec is a hash map / dictionary whose key-value pairs are stored (and can be iterated over) in a fixed order, by default the order in which they were inserted into the map. It's essentially a vector whose values can be inserted/retrieved with keys.

## Example
```rust
use hashvec::*;

// Create a new hashvec containing pairs of animal names and species
// The hashvec! macro acts like vec!, but with key-value tuple pairs
let mut hashvec: HashVec<&'static str, &'static str> = hashvec![
    ("Doug", "Kobold"),
    ("Skye", "Hyena"),
    ("Lee", "Shiba"),
    ("Sock", "Man"),
    ("Salad", "Wolf"),
    ("Finn", "Human")
];

// Insert a value into the hashvec (HashMap-style)
// Inserting overwrites existing keys' entries in-place
hashvec.insert("Jake", "Dog");

// Push a value onto the hashvec (Vector-style)
// Pushing overwrites existing keys' entries and moves them to the end
hashvec.push(("Susie", "Squid"));

// Access a value by key
match hashvec.get(&"Finn") {
    Some(value) => {
        assert_eq!(*value, "Human");
    },
    None => {}
}

// Access an entry by index
let lee_value = hashvec[2];
assert_eq!(lee_value, ("Lee", "Shiba"));

// Get the index of a key
let lee_index = hashvec.index(&"Lee").unwrap();
assert_eq!(lee_index, 2);

// Get the length of the hashvec
let hashvec_length = hashvec.len();
assert_eq!(hashvec_length, 8);

// Change an entry's key in-place
hashvec.rename(&"Salad", "Caesar");
assert_eq!(hashvec[4], ("Caesar", "Wolf"));

// Mutate a value
match hashvec.get_mut(&"Sock") {
    Some(value) => {
        *value = "Guinea Pig";
    },
    None => {}
}
assert_eq!(*hashvec.get(&"Sock").unwrap(), "Guinea Pig");

// Remove an entry
hashvec.remove_key(&"Doug");
assert_eq!(hashvec.get(&"Doug"), None);

// Swap the locations of two entries by their keys
hashvec.swap_keys(&"Lee", &"Skye");
assert_eq!(hashvec.index(&"Lee").unwrap(), 0);
assert_eq!(hashvec.index(&"Skye").unwrap(), 1);

// Now swap them again, by their indices
hashvec.swap_indices(0, 1);
assert_eq!(hashvec[0], ("Skye", "Hyena"));
assert_eq!(hashvec[1], ("Lee", "Shiba"));

// Iterate over each of the key-value pairs in the hashvec
for (k, v) in hashvec.into_iter() {
    println!("{} is a {}!", k, v);
}

// Remove an entry from the end of the hashvec
let last_entry = hashvec.pop();
assert_eq!(last_entry.unwrap(), ("Susie", "Squid"));

// Clear the hashvec
hashvec.clear();
```