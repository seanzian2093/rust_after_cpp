/// # Ch10.3 - Validating References with Lifetimes
/// * main aim of lifetimes is to prevent dangling references
/// * Rust borrow checker will compare scopes to determine whether all borrows are valid
///     * but mail fail to do so that is when we need lifetime parameter/specifiers
#[derive(Debug)]
pub struct Lifetimes{
}

impl Lifetimes{
    pub fn print(&self) {
        println!("\n======The note on lifetimes======");
    // Generic Lifetimes in Functions
        // - e.g., Rust can not tell the returned reference refers to x or y, we don't either
        // - we don't know concrete lifetimes of the references that will be passed in

        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }

        // - lifetime annotation in function signatures
            // - immediately after function name in angle brackets<>, just like generic type parameters
            // - e.g., in below function definition, we declare a lifet `'a` and add it to each reference
            // - this is to tell Rust to use below relationship in analyzing code 
                // - ref x and y live at least as long as lifetime `'a`
                // - return value lives same as the smaller of lifetimes of x and y 
            // - with these info, below code will compile
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
            // - use this function
                // - below code works
            let string1 = String::from("long string is long");
            {
                let string2 = String::from("xyz");
                let result = longest(string1.as_str(), string2.as_str());
                println!("The longest string is {}", result);
            }
                // - below does not work
                    // - because for `result` to be valid in the `println!` statement, `string2` would need to live untill after the `println1` statement
                        // - Rust knows this by the lifetime parameter `'a` in function signature
            // let result;
            // {
            //     let string2 = String::from("xyz");
            //     result = longest(string1.as_str(), string2.as_str());
            // }
            // println!("The longest string is {}", result);

    // Thinking in Terms of Lifetimes
        // - The way in which we need to specify lifetime parameter depends on what our function is doing
            // - e.g., below function return a single value x so lifetime of y does have any relationship with x or return value
        fn longest2<'a>(x: &'a str, y: &str) -> &'a str {x}
        // - when returning a ref from a function, the lifetime parameter for return value must match lifetime parameter for one of the parameters
            // - if not, it must refer to a value created within this function however this would be a dangling reference
    
    // Lifetime Annotations in Struct Definitions
        // - we can define structs to hold references but in this case we need to add lifetime annotation on every reference in definition
            // - e.g., similar to lifetime parameter in function, we define a generic lifetime parameter in <> after struct name
            // - the `'a` means an instance of `ImportantExcerpt` cannot live longer than `part`
        #[derive(Debug)]
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        
        fn main() {
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            let i = ImportantExcerpt {
                part: first_sentence,
            };
            println!("\ni is: {:?}", i);
        }
        main();
    // Three Elision Rules 
        // - compiler use to figure out lifetimes of ref when no explicit annotations
        // - first rule is compiler assigns a different lifetime parameter to each lifetime in each input type
            // - e.g., `fn foo(x: &i32)` becomes `fn foo<'a>(x: &'a i32)`
            // - e.g., `fn foo(x: &i32, y: &i32)` becomes `fn foo<'a>(x: &'a i32, y: &'b i32)`
            // - e.g., `fn foo(x: ImportantExcerpt)` becomes `fn foo(x: 'a ImportantExcerpt<'b>)`
        // - second rule is if there is exactly one INPUT lifetime parameter, that lifetime is aasigned to all OUTPUT lifetime parameter
            // - e.g., `fn foo<'a>(x: &'a i32) -> &'a i32`
        // - third rule is that if there is multiple INPUT lifetime parameters, but one of them is `&self` or `&mut self` because this is a method,
            // - then lifetime of `self` is assigned to all OUTPUT lifetime parameter 
    
    // Lifetime Annotations in Method Definitions
        // - Lifetime names for struct fields always need to be declared after the `impl` keyword and used after struct'a name
            // - because they are part of struct's type
        // - in method signature inside `impl` block, references could
            // - be tied to lifetime of references in struct's fields OR
            // - be independent

        // - e.g., declaration of `'a` after `impl` and its use after `ImportantExcerpt` are required
            // - based on the first elision rule, in `level` we do not need explicit annotation because compiler will do fo us 
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {3}
            // - based on third elision rule, both `&self` and `announcement` are given own lifetimes   
                // - and lifetime of `&self` is given to output `&str`
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    
    // The Static Lifetime
        // - `'static` denotes that affected references can live for the entire duration of the program
        // - all string literals have such lifetime, i.e., we can annotate as follows
            // - bacause text of string is stored directly in program's binary which is alway available
        let s: &'static str = "I have a static lifetime.";


            
    }
}