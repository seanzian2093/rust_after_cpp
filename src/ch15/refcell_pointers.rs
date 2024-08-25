//! Ch15.5 - `RefCell<T>` and the Interor Mutability Pattern
//!     * A recap of reasons to choose `Box<T>`, `Rc<T>`, and `RefCell<T>`
//!         * `Rc<T>` enables multiple owners of the same data; 
//!             * `Box<T>`, `RefCell<T>` have single owners
//!         * `Box<T>` allows immutable or mutable borrows checked at compile time
//!             * `Rc<T>` allows only immutable borrows checked at compile time
//!             * `RefCell<T>` allows immutable or mutable borrows checked at run time
//!                 * so we can mutate value inside `RefCell<T>` even when it is immutable
#[derive(Debug)]
#[allow(unused)]
pub struct RefCellPointers{
}

impl RefCellPointers{
    pub fn print(&self) {
        println!("\n======The note on RefCell smart pointer======");
    // Interior Mutability: A mutable borrow to an Immutable Value
    }
}
