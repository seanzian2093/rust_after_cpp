/// # Ch11.2 - Controlling How Tests Are Run
/// * control number of threads
///     * `cargo test -- --test-threads=1`
/// * control output of successful tests
///     * `cargo test -- --show-output`
/// * run a test by name
///     * `cargo test test1`
/// * run multiple tests by filtering
///     * `cargo run test` will run test on all test function names that contain with `test`
/// * ignore tests unless specifically requested
///     * requested by name or filtering
///     * e.g., `cargo test expensive_test`
///     * e.g., `cargo test expensive_`
///     * e.g., `cargo test expensive`

#[derive(Debug)]
pub struct ControllingTests{
}

impl ControllingTests{
    pub fn print(&self) {
        println!("\n======The note on controlling tests======");
    }
}


#[cfg(test)]
mod tests {
    #[test]
    // Add ignore attribute to ignore this test by default
        // - unless specifically requested
    #[ignore = "test"]
    fn expensive_test() {
        assert_eq!(2+2, 4);
    }
}