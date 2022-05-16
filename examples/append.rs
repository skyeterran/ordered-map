fn main() {
    use hashvec::*;

    let mut a: HashVec<&'static str, &'static str> = hashvec![
        ("Frank", "Dog"),
        ("Jimmy", "Pig")
    ];

    let mut b: HashVec<&'static str, &'static str> = hashvec![
        ("Mack", "Cat")
    ];

    a.append(&mut b);

    println!("{:#?}", a);
    println!("{:#?}", b);
}