fn main() {
    // In Rust variables are immutable by default
    // Immutable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable
    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 7;
    println!("The value of y is: {}", y);

    // Constants
    // Constants are always all UPPER CASE
    // You can never mutate a constant so you can be sure it will neer be annotated
    // const(s) must always be type annotated
    const SECRET: u32 = 69420;
    println!("The SECRET is: {}", SECRET);

    // Shadowing
    // Shadowing allows you to create a new variable with an existing name
    // This gives us 2 advantages: 1) We preserve mutability 2) We can change types
    let z = 8; // Immutable
    println!("The value of z is: {}", z);
    let z = 9; // Also Immutable
    println!("The value of z is: {}", z);
    let z = "Nine"; // i32 -> str
    println!("The value of z is: {}", z);
}
