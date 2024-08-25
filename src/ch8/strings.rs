/// # Ch8.2 - Storing UTF-8 Encoded Text with Strings
/// * Strings are implemented in RUst as a collectin of bytes
/// * plus some methods to provide useful functionality when those bytes are interpreted as text
/// * `str` is the only core type in Rust
/// * Rust standard library provides a `String` type
///     * growable, mutable, owned, UTF-8 encoded
///     * string slices are UTF-7 encoded too

#[derive(Debug)]
pub struct Strings {}

impl Strings{
    pub fn print(&self) {
        println!("\n======The note on Storing UTF-8 Encoded Text with Strings======");
    // Create a New String
        // - many Vec<T> operations available with `String` because it is a vector of bytes

        // - a new and empty String, mostly mutable
        let mut s1 = String::new();
        s1.push('H');
        s1.push_str("ello world");
        println!("\nAfter push and push_str, s1 is {s1}");

        // - from string literal
        let data = "initial contents";
            // - .to_string() works with any type that implements `Display`
        let s2 = data.to_string();
        println!("After to_string, s2 is {s2}");
            // - the method also works on a literal directly:
        let s3 = "initial contents".to_string();
        println!("After to_string, s3 is {s3}");
            // - or use String::from()
        let s4 = String::from("initial contents");
        println!("After String::from(), s4 is {s4}");
            // - works with any UTF-8 encode string literals
        let hello = String::from("السلام عليكم");
        println!("hell is: {hello}");
        let hello = String::from("Dobrý den");
        println!("hell is: {hello}");
        let hello = String::from("Hello");
        println!("hell is: {hello}");
        let hello = String::from("שָׁלוֹם");
        println!("hell is: {hello}");
        let hello = String::from("नमस्ते");
        println!("hell is: {hello}");
        let hello = String::from("こんにちは");
        println!("hell is: {hello}");
        let hello = String::from("안녕하세요");
        println!("hell is: {hello}");
        let hello = String::from("你好");
        println!("hell is: {hello}");
        let hello = String::from("Olá");
        println!("hell is: {hello}");
        let hello = String::from("Здравствуйте");
        println!("hell is: {hello}");
        let hello = String::from("Hola");
        println!("hell is: {hello}");

    // Updating a String
        // - size and cotents, just like Vec<T>
        // - use push_str to add string slice
        // - use push to add single char
        // - concatenate with + Operator or format! Macro
        let s5 = String::from("Hello, ");
        let s6 = String::from("world!");
            // - s1 has been moved here and can no longer be used
            // - + operator uses `add` method whose signature looks like
                // - fn add(self, s: &str) -> String {}
                // - compiler coerces &s6 which is &String into a &str
                    // - i.e., deref coerceion: &s6 to &s2[..]
        let s7 = s5 + &s6; 
        println!("\nAfter s5 + &s6, s6 is: {s6}");
        println!("After s5 + &s6, s7 is: {s7}");
        // - format! macro
            // - format! does not take ownership of any of its arguments
        let s8 = format!("{s7} - {s6}");
        println!("\nAfter format!, s8 is: {s8}");

    // Indexing into Strings
        // - Rust strings does not support direct indexing 
            // - e.g., below two statements raise error
        // let h1 = s8[0];
        // let h2 = "hello"[0];

        // - A String is a wrapper over a Vec<u8>
        let hello = String::from("Hola");
        println!("\n{hello}'s length: {}", hello.len());
        println!("\"{hello}\" is essentially {:?}", hello.as_bytes());
        let hello = String::from("Здравствуйте");
        println!("\n{hello}'s length: {}", hello.len());
        println!("\"{hello}\" is essentially {:?}", hello.as_bytes());
        
        // - View strings as bytes, scalar values, and grapheme clusters
        // - Slicing Strings
            // - better idea that indexing into a string
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("\nFirst 4 elements of \"{hello}\" is: {s}");

        // - Iterating Over Strings
            // - over character
        for c in hello.chars() {
            println!("{c}");
        }
            // - over bytes
        for b in hello.bytes() {
            println!("{b}");
        }
            



    }
}
