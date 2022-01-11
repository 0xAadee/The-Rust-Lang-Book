fn main() {
    // This chapter covers Rust's most unique feature - Ownership
    // Ownership allows Rust to make memory safety guarantees without the use of a garbage collector

    
    // What is ownership model
    // The ownership model is a way to manage memory
    
    // PROS
    // Control over memory
    // Error free
    // Faster runtime
    // Smaller progeam size

    // CONS
    // Slower write time.
    // Lurning curve (fighting with the borrow checker)

    // Rust is a memory safe programming language
    // It does this by doing bunch of compile time checks to make sure you are using memory in a safe way
    // Although Rust is a memory safe language by default, it does allow you to opt out of memory safety with the unsafe keyword. BUT USE IT SPARINGLY 
    // As you know everything in software is trade off, the ownership model gives us memory safety but the con is that we get a slower write time, slower than manual memory management. That's because Rust has a strict set of rules around memory management


    // Rust makes certain decisions based on if our memory is stored on the Stack or the Heap
    // During runtime our program has access to both the Stack and the Heap

    // Stack
    // The Stack is fixed and cannot grow or shrink during runtime
    // The Stack also stores Stack Frames, which are created gor every function that executes
    // and the Stack Frame stores the local variables of the function being executed
    // The size of a Stack Frame is calculated at compile time
    // That means the variables inside the Stack Frame must have a known fixed size
    // Variables inside of a Stack Frame only live as long as the Stack Frame lives
    fn a() {
        let x = "Hello";
        let y = 22;
        b();
    }

    fn b() {
        let x = String::from("World");
    }
    // Here a() gets executed first so we push a() onto the Stack
    // a() executes b() so we push the Stack Frame for b() on the Stack
    // Once b() finises executing it is popped off the Stack and all its variables get dropped
    // Once a() finishes all its local variables are dropped


    // Heap
    // On the other hand Heap is less organized
    // It can grow or shrink at runtime
    // The data stored in a heap can be dynamic in size
    // we control the lifetime of the data
    // x in b() is of type String, it can be dynamic is size
    // We can't directly store it on Stack, instead we ask the Heap to allocate memory for the String
    // Then the Heap passes back a pointer and the posinter is actually stored on the Stack
    // Pushing to the Stack is faster than allocating on the Heap
    // Because Heap has to spend time looking for a place to store the new data
    // It is faster to access data on the Stack because with the Heap you have to follow the pointer to follow the pointer
}
