//! Ch15.4 - `Rc<T>` the Reference Counted Smart Pointer
//!     * Use `Rc<T>` to enable multiple ownership which keeps track of the number of references to a value
//!         * to determine whether or not the value is still in use
//!     * Cloning an `Rc<T>` Increases the Reference Count
#[derive(Debug)]
#[allow(unused)]
pub struct RCPointers {}

#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::rc::Rc;
// Define a new struct with `Rc<T>`
#[derive(Debug)]
#[allow(unused)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

// use crate::ch15::rc_pointers::List::{Cons, Nil};
use List::{Cons, Nil};
use List2::{Cons2, Nil2};

#[allow(unused)]
impl RCPointers {
    pub fn print(&self) {
        println!("\n======The note on reference counted smart pointer======");

        // List b and c share List a so below code will not compile
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        // let c = Cons(4, Box::new(a));

        // List2 b and c share a
        let a2 = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
        println!("\ncount after creating a2 = {}", Rc::strong_count(&a2));
        // - `Rc::clone` makes a copy of the `Rc<T>`, i.e., a
        let b2 = Cons2(3, Rc::clone(&a2));
        println!("count after creating b2 = {}", Rc::strong_count(&a2));
        let c2 = Cons2(4, Rc::clone(&a2));
        println!("count after creating c2 = {}", Rc::strong_count(&a2));

        {
            let d2 = Cons2(4, Rc::clone(&a2));
            println!("count after creating c2 = {}", Rc::strong_count(&a2));
        }
        println!(
            "count after c2 goes out of scope = {}",
            Rc::strong_count(&a2)
        );

        println!("\na2 is {:?}", a2);
        println!("b2 is {:?}", b2);
        println!("c2 is {:?}", c2);
    }
}
