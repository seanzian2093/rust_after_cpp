/// # Ch14.1 - Customizing Builds with Release Profiles
/// * two main profiles that Cargo uses
///     * `dev` profile for `cargo build`, defines good defaults for development
///     * `release` profile for `cargo build --release`, defines good defaults for release builds 
/// * Cargo has default settings for each of above two profiles 
///     * when we have not explicitly added any `[profile.*]` section in `Cargo.toml` file
///     * when we do, we override any subsets of the default settings
///     * e.g., 
/// ```
/// [profile.dev]
/// opt-level = 0
/// [profile.release]
/// opt-level = 3
/// ```
