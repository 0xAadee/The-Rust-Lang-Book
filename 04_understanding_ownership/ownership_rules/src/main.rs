fn main() {
    // ----- Ownership rules -----
    // 1) Each value in Rust has a variable that's called its owner
    // 2) There can be only one owner at a time
    // 3) When the owner goes out of scope the value is dropped

    { // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // the scope is over and s is no longer valid
}
