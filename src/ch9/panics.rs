/// # Ch9.1 - Unrecoverable Errors with panic!
/// * by default, when a panic occurs, the program starts to unwinding, 
/// * i.e., Rust walks back up the stack and cleans up data from each function it encounters
/// * this is a lot of work. we can add below to Cargo.toml tile to switch from unwinding to abort immediately
/// ```
/// [profile.release]
/// panic = 'abort'
/// ```
/// * set `RUST_BACKTRACE=1` to enable backtraces of panics
#[derive(Debug)]
pub struct Panics {
}

impl Panics {
    pub fn print(&self) {
        println!("\n======The note on panic ======");
    }
}