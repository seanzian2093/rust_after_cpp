/// Ch11.3 - Organizing Tests
/// * Rust community thinks about test in terms of two main categories
///     * unit tests - small, more focued, testin one module in isolation at a time, can test private function
///     * integration tests - entirely external to your library and use your code in same way any other external code would
///         * i.e., public interfaces, exercising multiple modules per test
/// * Unit Test
///     * convention is to create a module named `tests`
///     * annotate the module with `#[cfg(test)]`
///         * `#[cfg(test)]` annotation on tests module tells rust to compile and run thetest code only when `cargo test` is run
///         * not when `cargo build`
///     * use `use super::*` to bring all parent module's item to scope
///     * Rust compiles these test code only when we explicitly run `cargo test`
/// * Integration Test
///     * need a `tests` directory at top level of project directory, peer to src
///         * create a `integration_test.rs` in this directory
///     * Rust knows to look for integration test files in this directory
///     * integration test will run only after all unittests passes
///     * run specific integration tests
///         * by function name `cargo test it_adds_two_ex`
///             * similar to run specific test of unit tests
///         * by file name `cargo test --test integration_test`
/// * Submodules in Intergration Test
///     * create a `mod.rs` file in `tests/common/`
///     * test functions in `tests/integration_test.rs` can use `common` as a module
#[derive(Debug)]
pub struct OrganizingTests{
}

impl OrganizingTests{
    pub fn print(&self) {
        println!("\n======The note on organizing tests======");
    }
}