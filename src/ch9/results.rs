/// # Ch9.2 - Recoverable Errors with Result
/// * Result is an Enum defined as
/// ```
/// enum Result<T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ```
/// * T represents the type of value that will be returned in a success case within Ok variant
/// * E reprensets the type of error that will be returned in a failure case whinin the Err variant

use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
#[derive(Debug)]
pub struct Results{
}

impl Results{
    pub fn print(&self) {
        println!("\n======The note on results======");
    
    // Simpe Match
        // - File::open returns a Result
        // - if the Result is an Err, panic!
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("\nProblem opening the file: {:?}", error),
        };
    
    // Matching on Different Errors

        let greeting_file_result = File::open("hello.txt");
        // - if the Result is an Err, match again
            // - if NotFound error then create the file and match
                // - Ok then bind the file to fc, and return it
                // - Err panic!
            // - for all other error, panic
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };

        // - use closre
            // - unwrap_or_else evaluate its argument lazily
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    
    // Shortcuts for panic on error: unwrap and expect
        // - Result.unwrap()
            // - if `Result` value is a `Ok` variant, `unwrap` returns the value inside `Ok`
            // - if `Err` variant, `unwrap` will call `panic!` macro
        // let greeting_file = File::open("hello_no_exist.txt").unwrap();

        // - Result.expect()
            // - similar to `unwrap` but let us choose error message for `panic!`
        let greeting_file = File::open("hello_no_exist.txt").expect("File not exists");
    
    // Propagating Errors
        // - return error to caller

        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
            // - explicitly return `Err(e)`
                Err(e) => return Err(e),
            };

            let mut username = String::new();

            // - implicitly return `Err(e)` because `match` block is an expression and last expression in block is the error
            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }

        // - a shortcut using `?` operator
            // - `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on
        fn read_username_from_file2() -> Result<String, io::Error> {
            // - `?` means that if `Ok`, value inside `Ok` will be returned from this expression and continue
            // - if `Err`, `Err` will be returned from the expresion and break, i.e., `Err` returned for whol function
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username)?;
            // - if all previous `Result`s are `Ok`, then return an `Ok` with `username` bound to it
            Ok(username)
        }
        // - more concise way
            // - chained
        fn read_username_from_file3() -> Result<String, io::Error> {
            let mut username = String::new();
            File::open("hello.txt")?.read_to_string(&mut username)?;
            Ok(username)
        }
            // - use `fs::read_to_string`
        fn read_username_from_file4() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
            
        // - `?` does not work here
            // - because main() returns ()
            // - but `File::open()` returns `Result<String, io::Error>`

        // fn main2() {
        //     let greeting_file = File::open("hello.txt")?;
        // }

    // `?` operator can be used for `Option` too
        // - similarly the return type requirement
        fn last_char_of_first_line(text: &str) -> Option<char> {
            // - whole function returns `Option`
            // - so does `next()`
            text.lines().next()?.chars().last()
        }

    // `main` can also return `Result`
        // - by default returns `()`


    }
}
