//! # Ch13.1 - Using Box<T> to Point to Data on the Heap
//! ## Refrence is the most common kind of pointer in Rust
//!     * only borrows data that they point to
//!     * no special capabilities, no overhead
//! 
//! ## Smart pointers are data stuctures that act like a pointer but
//!     * owns data that they point to
//!     * Have additional metadata and capabilities
//!     * String and Vec are smart pointer types
//!     * implemented using structs but with `Deref` and `Drop` traits
//! 
//! ## Box<T> is the most straightforward smart pointer
//!     * resides on stack but
//!     * points to data on heap, rather than stack
//!     * no overhead or extra capabilities
#[derive(Debug)]
#[allow(unused)]
pub struct PointersToHeap{
}

#[allow(unused)]
impl PointersToHeap{
    pub fn print(&self) {
        println!("\n======The note on pointers to heap======");
    // Using a Box<T> to Store Data on the Heap
        let b = Box::new(5);
        println!("\nb = {}", b);
        // - after this point both of below are deallocated
            // - the pointer, b, on the stack
            // - and the pointee, 5, on the heap

    // Using Box<T> to Store Recursive Type
        // - compiler can not know the size of recursive type at compile time
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
    
        use List::{Cons, Nil};
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}