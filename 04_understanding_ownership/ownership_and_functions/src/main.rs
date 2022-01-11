fn main() {
    let s = String::from("Hello");
    take_ownership(s);
    println!("{}", s);
    // s is moved - as passing a parameter into a function is same as assigning it to another variable
    // here passing in s moves it to some_string variable

    let x  = 5;
    makes_copy(x);
    println!("{}", x);
    // x gets copied as integers get copied and not moved
    // x is copied thats why we can still use it after the function call

    // It also works the opposite way
    let s1 = gives_ownership();
    println!("{}", s1);
    // Returning the String moves the ownership of the String to the s1 variable

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("world");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}