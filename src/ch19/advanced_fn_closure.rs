//! # Ch19.4 - Advanced Functions and Closures
//!     * Function Pointer
//!         * Functions coerce to type `fn`, i.e., function pointer,  not the `Fn` closure trait
//!             * a type, not a trait
//!         * passing functions with function pointers allow us to use functions as argument to other functions
//!         * Function pointers implement all three closure traits,i.e., `Fn`, `FnMut`, and `FnOnce`
//!             * so we can always pass a function pointer as an arugment for a function that expects a closure
//!         * Name of each enum variant is an initializer function, can be used as function pointers
//!     * Returning Closure
//!         * closures can not be returned direcly, compiler does not know how much space to allocate for closure
#[derive(Debug)]
#[allow(unused)]
pub struct AdvancedFnClosure {}

#[allow(unused)]
impl AdvancedFnClosure {
    pub fn print(&self) {
        println!("\n======The note on advanced functions and closures======");
        // Function pointer
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        // - do_twice's first parameter is a `fn` that takes one parameter of type `i32` and returns `i32`
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);
        println!("\nThe answer is: {}", answer);

        // Function pointer and closure as arugment to function
        let list_of_numbers = vec![1, 2, 3];
        // - use `map` to convert element of i32 to String in a vector
        // - `map` accepts function pointer or closure

        // - providing a closure
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        println!("\nlist_of_strings is {:?}", list_of_strings);

        // - providing a function, `to_string` from `ToString` trait
        let list_of_strings2: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
        println!("list_of_strings2 is {:?}", list_of_strings2);

        // Use enum variant as function pointer
        #[derive(Debug)]
        enum Status {
            Value(u32),
            Stop,
        }

        // - convert `u32` to `Value(u32)`
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
        println!("\nlist_of_statuses is {:?}", list_of_statuses);

        // Return a closre
        // - below code will not compile
        // fn returns_closure() -> dyn Fn(i32) -> i32 {|x| x + 1}
        // - use `Box`
        fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
