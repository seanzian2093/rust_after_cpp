/// # Ch8.1 - Storing Lists of Values with Vectors
/// * a collection type `Vec<T>`, aka vector.
/// * store more than one value in a single data structure that puts all values next to each other in memory,i.e. contiguous?
/// * vecotrs can only store values of same type
#[derive(Debug)]
pub struct Vectors{
}

impl Vectors{
    pub fn print(&self) {
        println!("\n======The note on storing lists of values with vectors======");

    // Create a New Vecotr
        // - e.g., v is an new and empty vector to hold values of type i32
        let mut v: Vec<i32> = Vec::new();
        println!("\nInitially v is {:?}", v);
            // - we can use push to add element when v is mutable
        v.push(1);
        v.push(2);
        v.push(3);
        println!("After push(), v is {:?}", v);
        // - use macro `vec![]` to create a new vector and initialize with values
            // - Rust will infer the type so no need to anotate type
            // - v1 does not need to be mutable
        let v1 = vec![1,2,3];
        println!("After defined with vec!, v1 is {:?}", v1);
    
    // Reading Elements of Vector
        // - via indexing
            // - panic at runtime if out of index, i.e. error
        let third1 = &v[2];
        println!("\nThe third element is {third1}");
        // let fourth= &v[3];
        // println!("\nThe fourth element is {fourth}");

        // - `get` method
            // - return a Option<T> struct
            // - easy handling of errous index
        let third2 = v.get(2);
        match third2 {
            Some(third3) => println!("Third element exists and is {third3}"),
            None => println!("Third element does not exist."),
        }
        let fourth2= v.get(3);
        match fourth2{
            Some(fourth2) => println!("Fourth element exists and is {fourth2}"),
            None => println!("Fourth element does not exist."),
        }

        // - reference to element borrows entire vector
            // - because elements of a vector are contiguous in memory
            // - changing one element may cause entire vector be deallocated and reallocated to new memory space
            // - ref to one element may points to deallocated memory if mutable ref exists in same scope

            // - e.g., _first_ref borrows v as immutable
        let _first_ref = &v[0];
            // - so push() is invalid, i.e., cannot borrow v as mutable ref because v is borrowed as immutable
            // - i.e, if compiled, after push, first may point to a freed memory - undefined behavior
        // v.push(4);
        println!("\nFirst element is: {_first_ref}");

        // - move out of vector, i.e, a collection
            // - below assignment works because i32 does not have heap data so v[0] is copied when assigning to first_own
        let first_own = v[0];
        println!("\nAfter first_own=v[0], first_own is {first_own:?}");
        println!("After first_own=v[0], v is {v:?}");

        let mut strings = vec![String::from("Hello ")];
            // - below assignment does not work because String has heap data so v[0] can not be copied without move
                // - .e.g, does not implement a Copy
                // - must use reference
        // let mut s=strings[0];
        let s= &mut strings[0];
        println!("\nAfter s =&strings[0], s is {s}");
        s.push_str(" world!");
        println!("After s.push_str(), s is {s}");
        println!("After s.push_str(), strings is {strings:?}");

    // Iterating over the Values in a Vector
        // - better to use `for` loop than index-based loop, i.e., one at a time
        // - over immutable ref
        println!("\n");
        for n_ref in &v {
            let n_plus_one = *n_ref + 1;
            println!("n_plus_one is {n_plus_one}");
        }

        // - over mutable ref
        for n_ref in &mut v {
            *n_ref += 1;
        }
        println!("After mutable loop, v is {:?}", v);
    // Dropping a Vector drops its elements.

    // Case Study #1
        // - v2 is a vecotr of mutable ref to i32
        let mut v2: Vec<&mut i32> = Vec::new();
        for i in &mut v {
            // - push(i) actually adding a mutable ref to i32 to v2
            v2.push(i);
        }
            // - v2[0] is a mutable ref to v[0]
                // - so changing *v2[0] is changing v[0]
        *v2[0] = 5;
        let a = *v2[0];
        let b = v[0];
        println!("\nAfter *v2[0]=5, a is: {a}, b is: {b}");

    }
}
