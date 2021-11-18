use colored::*;

fn main() {
    // Scalar data types
    // Scalar data types represent a single value
    // Rust has 4 main scalar data types
    println!("{}","Scalar Data Types".green());

    // Integers
    /*
    Length Signed Unsigned
    8-bit   i8     u8
    16-bit  i16    u16
    32-bit  i32    u32
    64-bit  i64    u64
    128-bit i128   u128
    arch    isize  usize
    */
    // arch depends on the architecture, usually 32 / 64 bit
    // Rust defaults to signed 32
    
    let a = 98_222; // Decimal (also 98222)
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary (also 0b11110000)
    let e = b'A'; // Byte (u8 only)

    // Integer overflow
    let f: u8 = 255; // u8 can hold max value of 255
    // Here if you go over 255, In Debug mode Rust will panic and in Release Builds Rust will perform two's complement wrapping, which means values greater than the maximum will wrap around to the minimum values, so 256 will become 0 and so on

    // Printing just to avoid warnings ;)
    println!("{}", "Integers".blue());
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
    println!("f = {}", f);


    // Floating-point numbers
    // The default is f64, a 64 bit double precision floating point number

    let g = 2.0;
    let h = 3.0;

    // Printing just to avoid warnings ;)
    println!("{}", "Floating-point numbers".blue());
    println!("g = {}", g);
    println!("h = {}", h);

    // Arithmatic Operations
    // addition
    let sum = 69 + 420;
    // subtraction
    let difference = 6.9 - 4.20;
    // multiplication
    let product = 6 * 9;
    // division
    let quotient = 20 / 4;
    // remainder
    let remainder = 420 % 69;

    // Printing just to avoid warnings ;)
    println!("{}", "Arithmatic Operations".blue());
    println!("69 + 420 = {}", sum);
    println!("6.9 - 4.20 = {}", difference);
    println!("6 * 9 = {}", product);
    println!("20 / 4 = {}", quotient);
    println!("420 % 69 = {}", remainder);
    

    // Booleans
    let tr = true;
    let fa = false;

    // Printing just to avoid warnings ;)
    println!("{}", "Booleans".blue());
    println!("tr = {}", tr);
    println!("fa = {}", fa);

    
    // Character
    let i = 'i'; // singe quoets ('') for char and double quotes ("") for string
    let big_i = 'I';
    let heart = '💗';

    // Printing just to avoid warnings ;)
    println!("{}", "Character".blue());
    println!("i = {}", i);
    println!("I = {}", big_i);
    println!("Heart = {}", heart);

    

    // Compound data types
    // Compound data types represent a group of values
    println!("{}", "Compound Data Types".green());

    // Tuple
    // Fixed size array of related data that could be of different types
    let tup = ("Elon", 69420);

    println!("{}", "Tuples".blue());

    // We can get values out of tuples in 2 ways:  1) Destructuring 2) Dot notation
    // Destructuring
    let (name, fav_num) = tup;
    println!("{}'s favourite number is {}", name, fav_num);
    // Dot notation
    let num = tup.1;
    println!("num = {}", num);


    // Arrays
    // Arrays in Rust are of fixed length
    let status_codes = [200, 404, 500];
    let not_found = status_codes[1];

    let byte = [0; 8]; // This line says create an Array with 8 values all set to 0

    // Printing just to avoid warnings ;)
    println!("{}", "Arrays".blue());
    println!("Not Found = {}", not_found);
    println!("value at 0th index of byte variable = {}",byte[0]);
}
