use hashvec::*;

fn main() {
    // Create a new hashvec containing pairs of animal names and species
    let mut hashvec: HashVec<&'static str, &'static str> = HashVec::new();

    // Insert values into the hashvec
    hashvec.insert("Doug", "Kobold");
    hashvec.insert("Skye", "Jaguar");
    hashvec.insert("Lee", "Shiba");
    hashvec.insert("Sock", "Man");
    hashvec.insert("Salad", "Dog");
    hashvec.insert("Finn", "Human");
    hashvec.insert("Jake", "Dog");
    
    // Access a value by key
    match hashvec.get("Finn") {
        Some(value) => {
            assert_eq!(*value, "Human");
        },
        None => {}
    }

    // Access an entry by index
    let lee_value = hashvec[2];
    assert_eq!(lee_value, ("Lee", "Shiba"));

    // Get the index of a key
    let lee_index = hashvec.index("Lee").unwrap();
    assert_eq!(lee_index, 2);
    
    // Mutate a value
    match hashvec.get_mut("Sock") {
        Some(value) => {
            *value = "Guinea Pig";
        },
        None => {}
    }
    assert_eq!(*hashvec.get("Sock").unwrap(), "Guinea Pig");

    // Remove a value
    hashvec.remove("Doug");
    assert_eq!(hashvec.get("Doug"), None);
    
    // Iterate over each of the key-value pairs in the hashvec
    for (k, v) in hashvec.into_iter() {
        println!("{} is a {}!", k, v);
    }
    
    // Clear the hashvec
    hashvec.clear();
}