//! # Ch13.3 - Running Code on Cleanup with the `Drop` Trait
//!     * Functionality of `Drop` trait is almost always used when implementing a smart pointer
//!     * Implementing the `Drop` trait to specify what code to run when a value goes out of scope
//!         * `Drop` is included in prelude
//!     * Drop a value early with `std::mem:drop`
//!         * fn `drop` in `Drop` trait is not allowed to be called explicitly
//!             * to avoid double free issue when `Drop` is automatically run
//!             * we can not disable it
//!         * use `drop(obj)` to drop obj
//!             * `drop` is included in prelude

#[derive(Debug)]
#[allow(unused)]
pub struct DropTrait {}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[allow(unused)]
impl DropTrait {
    pub fn print(&self) {
        println!("\n======The note on drop trait======");

        // A smart pointer that implements `Drop` trait
        // - show when `Drop` is run automatically
        let c = CustomSmartPointer {
            data: String::from("stuff c"),
        };

        let d = CustomSmartPointer {
            data: String::from("stuff d"),
        };

        let e = CustomSmartPointer {
            data: String::from("stuff e"),
        };

        // Use `std::mem::drop` to drop an object earlier
        println!("CustomSmartPointers created.");
        // - `drop` in `Drop` trait is not allowed to be called explicitly
        // c.drop();
        // - use `std::mem::drop` instead
        drop(c);
        // - d and e will be dropped after this println!
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
