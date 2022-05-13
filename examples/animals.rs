use ordered_map::OrderedMap;

fn main() {
    let mut map: OrderedMap<&'static str, &'static str> = OrderedMap::new();
    map.insert("Doug", "Kobold");
    map.insert("Skye", "Jaguar");
    map.insert("Lee", "Shiba");
    map.insert("Sock", "Man");
    map.insert("Salad", "Dog");
    map.insert("Salad", "Coyote");
    map.insert("Lee", "Human");

    println!("{:#?}", map);

    for (k, v) in map.into_iter() {
        println!("{}: {}", k, v);
    }
}