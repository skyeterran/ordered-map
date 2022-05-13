use ordered_map::OrderedMap;

fn main() {
    let mut map: OrderedMap<&'static str, &'static str> = OrderedMap::new();
    map.insert("Doug", "Kobold");
    map.insert("Skye", "Jaguar");
    map.insert("Lee", "Shiba");
    map.insert("Sock", "Man");
    map.insert("Salad", "Dog");
    map.insert("Lee", "Human");

    println!("{:#?}", map);

    for (k, v) in map.into_iter() {
        println!("{}: {}", k, v);
    }

    // Change the kind of animal that Sock is
    match map.get_mut("Sock") {
        Some(value) => {
            *value = "Guinea Pig";
        },
        None => {}
    }

    println!("{:#?}", map);

    for (k, v) in map.into_iter() {
        println!("{}: {}", k, v);
    }
}