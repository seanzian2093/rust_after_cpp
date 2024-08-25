// Ch4.1 - What Is Ownership?
#[derive(Debug)]
pub struct Ownership{
}

impl Ownership{
    pub fn print(&self) {
        println!("\n======The note of ownership======");

    // Variable Live in the Stack
        // - variables live in frames
            // - a frame is a mapping from variable to values within a single scope, such as a function.
        // - Frames are organized into a stack of currently-called-functions.
            // - After a function returns, Rust deallocates the function's frame, aka, freeing, dropping
            // - the most recent frame added is always next frame freed
        let a = 5;
        println!("a is now: {a}");
        let mut b = a;
        println!("After asigned to b, a is now: {a}, b is {b}");
        b += 1;
        println!("After increment 1 to b, a is now: {a}, b is {b}");
    // Boxes Live in the Heap
        // - heap is a separate region of memory where data can live indefinitely.
            // - not tied to a specific stack frame
        // - Rust provides a construct called `Box` for putting data on the heap
        // - Use pointer to indicate the allocated memory in the heap
            // - value that a pointer points to is called pointee

    // Rust Does Not Permit Manual Memory Management
        // - not function like free, or delete
    
    // A Box's Owner Manages Deallocation
        // - Rust automatically frees a box's heap memory, i.e.
        // - if a variable owns a box, when Rust deallocates the variable's frame, Rust deallocates the box's heap memory
    
    // Collections Use Boxes
        // - Boxes are used by Rust data structures like, `Vec`, `String`, `HashMap` to hold a variable number of elements
            // - now first owns "Ferris"
        let first = String::from("Ferris");
            // - since add_suffix takes a String as parameter, not a ref, so it takes ownership, i.e., first is moved 
                // - a String is not copied because String does not implement a Copy trait
                // - other types may be copied in such case, i.e. i32, f64
        let full = add_suffix(first);

    // Variables Cannot Be Used After Being Moved
        println!("\nAfter add_suffix(), full is now: {full}");
            // - since first is moved 
        // println!("{first}");
        fn add_suffix(mut name: String) -> String {
            name.push_str(" Jr.");
            name
        }
    
    // Cloning Avoids Moves
        // - use .clone() method to clone to avoid moves
        let first = String::from("Ferris");
        let first_clone = first.clone();
        let full = add_suffix(first_clone);
        // - first is still usable
        println!("\nUsing clone, full is: {full}, first is still available: {first}");
        
    // Summary of Ownership
        // - all heap data must be owned by exactly one variable
        // - Rust deallocates heap data once its owner goes out of scope
        // - Ownership can be transferred by moves, which happen on assignments and function calls.
        // - Heap data can only be accessed through its current owner, not a previous owner.
    }
}