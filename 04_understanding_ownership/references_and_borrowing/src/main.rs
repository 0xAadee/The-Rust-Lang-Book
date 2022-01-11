fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_bad(s1);
    println!("The length of '{}' is {}", s2, len);

    let s3 = String::from("hello");
    let len = calculate_length(&s3); // Passing in references as function parameters is called borrowing
    println!("The length of '{}' is {}", s3, len);
    // References are immutable by default

    let mut s4 = String::from("hello");
    change(&mut s4);
    // You can have only one mutable reference to a particular peice of data in a particular scope
    // This allows Rust to avoid data races
    // You can't have a mutable reference to a particular peice of data in a particular scope if a immutable reference already exists
    // This is because immutable references don't expect the underlying value to change
    // Howerever you can have multiple immutable references

    // The scope of a reference starts when its first introduced and ends when its last used

    // Dangling references (Reference that points to invalid data)
    let reference_to_nothing = dangle();


    // The Rules of References
    // 1) At any given time, you can have either one mutable reference or any number of immutable references
    // 2) References must always be valid
}

fn calculate_length_bad(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to the String and references don't take the ownership of the underlying value
    // References are immutable by default
    let length = s.len();
    length
}

fn change(s: &mut String) {
    s.push_str(" world!");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s

    // Rust stops us from referencing invalid databy throwing an error
}