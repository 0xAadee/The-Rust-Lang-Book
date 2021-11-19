fn main() {
    println!("{}", my_funtion(11, 12));
    println!("{}", my_function_two(24, 45));
}

// In Rust we can think about a peice of code as Statement or Expression
// Statements perform some function but do not return a value
// Expressions return a value

// You need to specify a return type for a function by adding '->' and the data type after parantheses
fn my_funtion(x: i32, y: i32) -> i32 {
    // Here Printline statements are Statements because they don't return any value
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    // Here 'x + y' is an expression because it returns a value
    let sum = x + y;

    // In Rust we can return values from a funtion in 2 ways
    // 1) The return statement
    return sum;
}

fn my_function_two(x: i32, y: i32) -> i32 {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    // In Rust we can return values from a funtion in 2 ways
    // 2) The last expression of a funtion is returned implicitly
    // And for the last expression we omit the semi colon (;)
    x + y
}
