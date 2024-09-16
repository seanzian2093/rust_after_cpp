/// # Ch11.1 - How to Writing Tests
#[derive(Debug)]
pub struct WritingTests {}

#[allow(dead_code, unused_variables)]
impl WritingTests {
    pub fn print(&self) {
        println!("\n======The note on writing tests======");
    }

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    fn between_one_hundred(value: i32) -> i32 {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        value
    }

    fn use_result(a: i32) -> Result<(), String> {
        if WritingTests::add_two(a) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::ch11::writing_tests::WritingTests;
    use super::*;
    // Define a test function
    // - this `#[test]` tells Rust that this is a test function and will be run with `cargo test` command
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[ignore = "test"]
    fn another() {
        panic!("Make this test fail");
    }

    // Checking Results with the `assert!` Macro
    // - test a condition evaluates to `true` or not
    #[test]
    fn test_assert() {
        assert!(1 < 2);
    }

    // Testing Equality with `assert_eq!` and `assert_ne` Macros
    #[test]
    fn it_adds_two() {
        // assert_eq!(4, crate::ch11::writing_tests::WritingTests::add_two(2));
        assert_eq!(4, WritingTests::add_two(2));
    }
    #[test]
    fn it_adds_two2() {
        assert_ne!(5, WritingTests::add_two(2));
    }

    // Adding Custom Failure Messages
    #[test]
    #[ignore = "test"]
    fn it_adds_two3() {
        let res = WritingTests::add_two(2);
        let expected = 5;
        assert_eq!(expected, res, "\nExpected is {expected}, result is {res}");
    }

    // Checking `panic` with `should_panic` attribute
    #[test]
    #[should_panic]
    fn greater_than_100() {
        WritingTests::between_one_hundred(200);
    }

    // Using Result<T,E> in Tests
    // - cannot use `#[should_panic]` annotation on tests that use `Result<T,E>`
    // - use `?` operator to return `Err` for test function to fail
    // - error message bound to `Err` variant will be printed out
    #[test]
    fn test_result() -> Result<(), String> {
        let res = WritingTests::use_result(2)?;
        Ok(res)
    }
    // - to assert an operation returns `Err` variant, use`assert!(value.is_err())`
    #[test]
    fn test_is_err() {
        let res = WritingTests::use_result(4);
        assert!(res.is_err());
    }
}
