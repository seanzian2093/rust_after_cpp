//! # Ch19.3 - Advanced Types
//!     * Newtype patter, discussed in Ch19.2
//!     * Creating Type Synonyms with Type Aliases
//!     * the Never Type that never returns
//!         * Rust has a special type named `!` known as empty type because it has no values
//!         * we prefer to call it never type because it stands in the place of return type when a function will never return
//!         * `continue` expression has the type `!`
//!         * `panic!` macro has the type `!`
//!         * a `loop` expression without a `break` statement has type `!`
//!     * Dynamically Sized Types and the `Sized` Trait
//!         * Rust needs to know at compile time certain details about its type e.g., how much space to allocate for a value of a type
//!         * Dynamically sized types, aka DSTs, unsized types whose size we know only at runtime
//!             * e.g., `str` is a DST
//!         * to work with DST, Rust provides the `Sized` trait to determine whether or not a type's size is known at compile time
//!             * automatically implemented for everything whose size is known at compile time
//!
#[derive(Debug)]
#[allow(unused)]
pub struct AdvancedTypes {}

#[allow(unused)]
impl AdvancedTypes {
    pub fn print(&self) {
        println!("\n======The note on advanced types======");
        // Creating Type Synonyms with Type Aliases
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("\nx + y = {}", x + y);
        // - to reduce repeatition of long code
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));

        // fn takes_long_type(f: Thunk) {}
        // fn returns_long_type() -> Thunk {}

        // Never type
        // - bar is a function that never returns

        // fn bar() -> ! {
        //     // - snip
        // }

        // - `continue` expression has the type `!`
        // - so below match expression is guaranteed to return `u32` because match requires all arms return same type

        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        // Dynamicall Sized Types
        // - size of `str` can not be known at compile time so does compiler complain

        // let s1: str = "Hello there!";
        // let s2: str = "How's it going?";

        // - `Sized` trait is automatically implemented for those whose size is know at compile time
        // - so a generic function defined like this

        // fn generic<T: Sized>(t: T) {
        //     // --snip--
        // }
        // - is actually treated as though we had written this

        // fn generic<T: Sized>(t: T) {
        //     // --snip--
        // }
    }
}
