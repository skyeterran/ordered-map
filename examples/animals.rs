use ordered_map::*;

fn main() {
    // Create a new map containing pairs of animal names and species
    let mut map: OrderedMap<&'static str, &'static str> = OrderedMap::new();

    // Insert values into the map
    map.insert("Doug", "Kobold");
    map.insert("Skye", "Jaguar");
    map.insert("Lee", "Shiba");
    map.insert("Sock", "Man");
    map.insert("Salad", "Dog");
    map.insert("Finn", "Human");
    map.insert("Jake", "Dog");
    
    // Access a value by key
    match map.get("Finn") {
        Some(value) => {
            assert_eq!(*value, "Human");
        },
        None => {}
    }

    // Access an entry by index
    let lee_value = map[2];
    assert_eq!(lee_value, ("Lee", "Shiba"));

    // Get the index of a key
    let lee_index = map.index("Lee").unwrap();
    assert_eq!(lee_index, 2);
    
    // Mutate a value
    match map.get_mut("Sock") {
        Some(value) => {
            *value = "Guinea Pig";
        },
        None => {}
    }
    assert_eq!(*map.get("Sock").unwrap(), "Guinea Pig");

    // Remove a value
    map.remove("Doug");
    assert_eq!(map.get("Doug"), None);
    
    // Iterate over each of the key-value pairs in the map
    for (k, v) in map.into_iter() {
        println!("{} is a {}!", k, v);
    }
    
    // Clear the map
    map.clear();
}