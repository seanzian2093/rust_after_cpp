//! # Ch19.5 - Macros
//!     * Macros expand to produce more code than we have written manually, i.e., metaprogramming 
//!         * a macro can take variable number of parameters
//!         * macros are expanded before compiler interprets the meaning of the code
//!         * macros must be defined and brought to scope before calling them in a file
//!     * Declarative Macros with `macro_rules!` for General Metaprogramming
//!         * match against pattern and 
//!         * replace code with other code
//!     * Procedural Macros for Generating Code from Attributes
//!         * second form of macros is procedural macro
//!         * accept some code as input, operate on that code, and produce code as output
//!         * three kinds of procedural macros
//!             * custom drive, attibute-like, and function-like
//!         * when creating procedural macros, the definitions must reside in their own crate with a special crate type

// Define a declarative macro
    // - `#[macro_export]` annotation indicates that this macro should be made available whenever 
        // - the crate in which this macro is defined is brought into scope
        // - wihout this annotation, the macro cannot be brought into scope
#[macro_export]
    // - start the definition with `macro_rules!` and the name of the macro, i.e. `vec`
macro_rules! vec {
    // - one arm with pattern `( $( $x:expr ),* )`, followed by `=>` and a block of code associated with the pattern
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Define a procedural macro
// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
// }



#[derive(Debug)]
#[allow(unused)]
pub struct Macros{
}

#[allow(unused)]
impl Macros{
    pub fn print(&self) {
        println!("\n======The note on macros======");
    }
}