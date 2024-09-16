//! Ch19.1 - Unsafe Rust
//!     * Unsafe super power
//!         * Dereference a raw pointer
//!         * Call an unsafe function or method
//!         * Access or modify a mutable static variable
//!         * Implement an unsafe trait
//!         * Access fields of `union`s
//!     * Two types of Raw Point
//!         * `*const T`, i.e., immutable, i.e., cannot be directly assigned to after being dereferenced
//!         * `*mut T`, i.e., mutable
//!
//!         * `*` is not the dereference operator but part of the type name
//!         * Power of raw pointer
//!             * are allowed to ignore the borrowing rule by having both immutable and mutable pointers or multiple mutable pointers to the same location
//!             * are not guaranteed to point to valid memory
//!             * are allowed to be null
//!             * don't implement any automatic cleanup

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

// Declare an unsafe trait
unsafe trait Foo {
    // methods go here
}

// Implement an unsafe trait
unsafe impl Foo for i32 {
    // method implementations go here
}

#[derive(Debug)]
#[allow(unused)]
pub struct UnsafeRust {}

#[allow(unused)]
impl UnsafeRust {
    pub fn print(&self) {
        println!("\n======The note on unsafe Rust======");
        // Raw pointer from ref
        // - create in safe code
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        // - we must dereference a raw pointer in a `unsafe` block
        unsafe {
            println!("\nr1 is {}", *r1);
            println!("r2 is {}", *r2);
        }

        // Calling an Unsafe Function or Method
        unsafe fn dangerous() {}
        unsafe {
            dangerous();
        }

        // Using `extern` Functions to Call External Code
        // - import `abs` from `C`
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        // - functions imported from other languanges are always unsafe
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }

        // Accessing or Modifying a Mutable Static Variable
        // - `HELLO_WORLD` is an immutable static variable
        println!("name is: {}", HELLO_WORLD);

        // - `COUNTER` is a mutable static variable
        // - modify it in unsafe code
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);

        unsafe {
            // - access it in unsafe code
            println!("COUNTER: {}", COUNTER);
        }
    }
}
