//! # Ch13.2 - Treating Smar Pointers Like Regular References with the Deref Trait
//!     * `Deref` trait is implemented is such a way that a smart pointer can be treated like a regular reference
//!     * Implicit Deref Coercions with Functions and Methods
//!         * Deref coercion converts
//!             * a ref to a type that implements `Deref` trait into
//!             * a ref to another type
//!         * e.g., `&String` to `&str` because `&String` implement `Deref`
//!     * How Deref Coercion Interacts with Mutability
//!         * From `&T` to `&U` when `T`: `Deref<Target=U>`
//!         * From `&mut T` to `&mut U` when `T`: `DerefMut<Target=U>`
//!         * From `&mut T` to `&U` when `T`: `Deref<Target=U>`
//!             * not reverse
#[derive(Debug)]
#[allow(unused)]
pub struct DerefTrait {}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // - access first element of a tuple struct
        &self.0
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
struct AccessLogger(i32);
impl Deref for AccessLogger {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}

impl DerefTrait {
    pub fn print(&self) {
        println!("\n======The note on Deref trait======");
        // Following the Pointer to the Value
        let x = 5;
        // - y is a regular ref to x
        let y = &x;
        // - y2 is a smart pointer, points to a copy of x, stored in heap
        let y2 = Box::new(x);

        assert_eq!(5, x);
        // - use `*` operator to get the pointee of a regular ref
        assert_eq!(5, *y);
        // - use `*` operator to get the pointee of a smart pointer
        assert_eq!(5, *y2);

        // Define Our Own Smart Pointer
        let y3 = MyBox::new(x);
        // - use `*` operator to get the pointee of MyBox
        // - actually running `*(y.deref())`
        assert_eq!(5, *y3);

        // Deref coercion
        // - fn hello takes &str as argument
        fn hello(name: &str) {
            println!("Hello, {name}!");
        }
        hello("Rust");
        // - when we provide `MyBox` which implements `Deref`, Rust does following steps for us automatically
        // - deref `MyBox` to `String` so `&MyBox` to `&String`, and `String` implements `Deref` too so
        // - Rust deref `&String` to `&str`
        // - Rust will do as many times as needed to get the matching type
        // - at compile time, so no runtime penalty
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        // - or we could manually do this ourselves
        // - get the `String` which is pointee of m by *m
        // - get a string slice, i.e., str, by indexing, by (*m)[..]
        // - get &str by borrowing the str with `&`
        hello(&((*m)[..]));
    }
}
