fn main() {
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)

    let s3  = String::from("world");
    let s4 = s3.clone(); // Copied
}
