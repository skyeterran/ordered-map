use ordered_map::*;

fn main() {
    // Create a new map
    let mut map: OrderedMap<&'static str, &'static str> = OrderedMap::new();

    // Insert values into the map
    map.insert("Doug", "Kobold");
    map.insert("Skye", "Jaguar");
    map.insert("Lee", "Shiba");
    map.insert("Sock", "Man");
    map.insert("Salad", "Dog");
    map.insert("Finn", "Human");
    map.insert("Jake", "Dog");
    
    // Access a value
    match map.get("Finn") {
        Some(value) => {
            assert_eq!(*value, "Human");
        },
        None => {}
    }
    
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
    
    for (k, v) in map.into_iter() {
        println!("{}: {}", k, v);
    }
    println!("{:?}", map);
    
    // Clear the map
    map.clear();
}