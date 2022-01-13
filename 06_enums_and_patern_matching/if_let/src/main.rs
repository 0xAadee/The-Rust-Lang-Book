fn main() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (), // _ means everything else
    }

    // if let syntax is not exhaustive, that means we only have to specify the paterns we care about and all others are ignored
    if let Some(3) = some_value {
        println!("three");
    }
}
