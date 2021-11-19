fn main() {
    // if-else
    //  1
    let number = 5;

    // In Rust the expression must be explicitly a boolean
    if number < 10 {
        println!("First condition was true!");
    } else if number < 22 {
        println!("Second condition was true!");
    } else {
        println!("Condition was false!");
    }

    // 2
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);

    // Loops
    // loop
    // The simplest type of loop in Rust is 'loop', it will run until we call break
    let mut counter = 0;
    // We can also return values
    let result = loop {
        counter += 1;
        println!("{}", counter);

        if counter == 10 {
            // adding return value after break will break and return the value
            break counter;
        }
    };

    println!("The result is {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    // for loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("{}", element);
    }

    // for in range
    for i in 1..4 {
        // 1..4 => 1, 2, 3
        println!("{}", i);
    }
}
