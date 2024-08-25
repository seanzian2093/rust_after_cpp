/// # Ch4.4 - The Slice Type
#[derive(Debug)]
pub struct SliceType{
}
/// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
/// A slice is a kind of reference, so it is non-owning pointer


impl SliceType{
    pub fn print(&self) {
        println!("\n======The note of Slice Type======");
    // String slice
        // - is a &str type
        fn string_slice() {
            let s = String::from("hello world");
            
            let hello: &str = &s[0..5];
            let world: &str = &s[6..11];
            let s2: &String = &s; 

            // - since slices are reference, ownership rules applies - no immutable ref allowed now, as .push requires
            // s.push('!');
            println!("\ns is {s}, hello is {hello}, world is {world}, and s2 is {s2}");
        }
        string_slice();

    // Range Syntax
        fn range_syntax() {
            let s = String::from("hello");
            // - if start from begining, i.e. 0, start can be omitted
            let slice1 = &s[0..2];
            let slice2 = &s[..2];
            assert!(slice1 == slice2);
            // - if includes the last byte of String, end can be omitted
            let len = s.len();
            let slice3 = &s[3..len];
            let slice4 = &s[3..];
            assert!(slice3 == slice4);
        }
        range_syntax();

    // String Literals Are Slices
        // - String literal is of type &str, i.e. a slice pointing to that specific point of the binary, an immutable reference
    // String Slices as Parameters
        // - first_word takes &str as parameter, we can pass
            // - string slice since it is &str type
            // - a ref to String because of deref of coercions, discussed ch15
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
        
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
        
            &s[..]
        }
        // - taking String slice
        let my_string = String::from("hello world");

        let word = first_word(&my_string[0..6]);
        println!("\nfirst_word of my_string[0..6] is {word}");

        let word = first_word(&my_string[..]);
        println!("first_word of my_string[..] is {word}");

        let word = first_word(&my_string);
        println!("first_word of my_string is {word}");

        // - taking string literal and slice
        let my_string_l = "hello world";

        let word = first_word(&my_string_l[0..6]);
        println!("\nfirst_word of my_string_l[0..6] is {word}");

        let word = first_word(&my_string_l[..]);
        println!("first_word of my_string_l[..] is {word}");

        let word = first_word(&my_string_l);
        println!("first_word of my_string_l is {word}");

    // Other Slice
        fn other_slice() {
            let a = [1, 2, 3, 4, 5];
            let slice = &a[1..3];
            assert_eq!(slice, &[2, 3]);
        }
        other_slice();
    }
}

