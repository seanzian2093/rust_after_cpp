// Ch4.3 - Fix Ownership Error
#[derive(Debug)]
pub struct FixOwnershipError{
}

impl FixOwnershipError{
    pub fn print(&self) {
        println!("\n======The note of Fix Ownership Error======");

    // Fixing an Unsafe Program: Returning a Reference to the Stack
        // - below is an unsafe program, i.e., compiler does allow returning a ref to local variable

        // fn return_a_string() -> &String {
        //     let s = String::from("Hello world");
        //     &s
        // }

        // - remedy 1 - move ownership of the string out of the function
            // - return the string, not a ref to it
        fn return_a_string1() -> String {
            let s = String::from("Hello world");
            s
        }
        let res1 = return_a_string1();
        println!("\nResult of return_a_string1() is: {res1}");

        // - remedy 2 - return a string literal
            // - which lives forever but only applies if we intend not to change the string so heap allocation is not necessary
        fn return_a_string2() -> &'static str{
            "Hello world"
        }
        let res2 = return_a_string2();
        println!("Result of return_a_string2() is: {res2}");

        // - remedy 3 - defer borrow-checking to runtime by using garbage collection
            // - more details in ch15
        use std::rc::Rc;
        fn return_a_string3() -> Rc<String> {
            let s = Rc::new(String::from("Hello world"));
            Rc::clone(&s)
        }
        let res3 = return_a_string3();
        println!("Result of return_a_string3() is: {res3}");

        // - remody 4 - have the caller provide a slot to put the string
            // - using mutable ref
        fn return_a_string4(output: &mut String) {
            output.replace_range(.., "Hello world");
        }
        let mut output = String::new();
        return_a_string4(&mut output);
        println!("Result of return_a_string4() is: {output}");

    // Fixing an Unsafe Program: Not Enough Permissions
        // - below is an unsafe program trying to achieve ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
            // - name is an immutable ref
            // - but push may invalidate other references to name, outside of stringify_name_with_title

        // fn stringify_name_with_title(name: &Vec<String>) -> String {
        //     name.push(String::from("Esq."));
        //     let full = name.join(" ");
        //     full
        // }


        // - remedy 1 - change type of parameter name to mutable ref
            // - but genrally functions should not mutate their inputs if caller would not expect it
        fn stringify_name_with_title1(name: &mut Vec<String>) -> String {
            name.push(String::from("Esq."));
            let full = name.join(" ");
            full
        }

        let mut name = vec![String::from("Ferris")];
        let full = stringify_name_with_title1(&mut name);
        println!("\nResult of stringify_name_with_title1 is: {}", full);
        println!("After stringify_name_with_title1 name is: {:?}", name);
        
        // - remedy 2 - take ownership of name
            // - and return the ownership
            // - rare that Rust functions take ownership of heap-owning data structure like Vec and String
        fn stringify_name_with_title2(mut name: Vec<String>) -> (Vec<String>, String) {
            name.push(String::from("Baron."));
            let full = name.join(" ");
            (name, full)
        }
            // - old name is unusable afterward unless we return it and assign to the same variable name
        let (name, full) = stringify_name_with_title2(name);
        println!("\nResult of stringify_name_with_title2 is: {}", full);
        println!("After stringify_name_with_title2 name is: {:?}", name);
        
        // - remedy 3 - change parameter name to immutable ref
            // - and clone it in function scope
         
        fn stringify_name_with_title3(name: &Vec<String>) -> String {
            let mut name_clone = name.clone();
            name_clone.push(String::from("Duke"));
            let full = name_clone.join(" ");
            full
        }
        let full = stringify_name_with_title3(&name);
        println!("\nResult of stringify_name_with_title3 is: {}", full);
            // - name is unchanged
        println!("After stringify_name_with_title3 name is: {:?}", name);
        
    // Fixing an Unsafe Program: Aliasing and Mutating a Data Structure
        // - below is an unsafe program because
            // - largest is an immutable ref to dst while dst.push() requires a mutable ref to dst, i.e. aliasing and mutating simultaneously
            // - thus rejected by compiler
            // - key signt to fix this problem is shorten the lifetime of largest to not overlap with dst.push()

        // fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
        //     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
        //     for s in src {
        //         if s.len() > largest.len() {
        //             dst.push(s.clone());
        //         }
        //     }
        // }

        // - remedy 1 - clone largest
            // - so largest is not a ref to String but String
        fn add_big_strings1(dst: &mut Vec<String>, src: &[String]) {
            let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
            for s in src {
                if s.len() > largest.len() {
                    dst.push(s.clone());
                }
            }
        }
            // - when using into(), must explicitly annotate type in declaration
        let mut dst: Vec<String> = vec!["a".into(), "bc".into(), "def".into()];
        let src: Vec<String> = vec!["ghil".into(), "klm".into(), "nopq".into()];
        add_big_strings1(&mut dst, &src);
        println!("\nAfter add_big_strings1, the dst is: {:?}", dst);

        // - remedy 2 - complete comparison before mutation
        fn add_big_strings2(dst: &mut Vec<String>, src: &[String]) {
            let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
            let to_add: Vec<String> = src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
            dst.extend(to_add);
        }
        let src: Vec<String> = vec!["rstu".into(), "vwxyz".into(), "123".into()];
        add_big_strings2(&mut dst, &src);
        println!("After add_big_strings2, the dst is: {:?}", dst);

        // - remedy 2 - copy largest len and compare
            // - since we do not need the content of String ,just their length
        fn add_big_strings3(dst: &mut Vec<String>, src: &[String]) {
            let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
            for s in src {
                if s.len() > largest_len {
                    dst.push(s.clone());
                }
            }
        }
        let src: Vec<String> = vec!["456789".into(), "01".into(), "123".into()];
        add_big_strings3(&mut dst, &src);
        println!("After add_big_strings3, the dst is: {:?}", dst);

    // Fixing an Unsafe Program: Copying vs. Moving out of a Collection
        // - if a value does not own heap data, then it can be copied without a move
            // - i32 does not own heap
            // - String own heap
            // - &String does not own heap
            // - except mutable reference cannot be copied. This prevents two mutable references to same data from being used at same time

        // - below program works because var is at the stock
            // - becasue i32 has Copy trait
        fn move_out_of_collection() {
            let v: Vec<i32> = vec![0, 1, 2];
            let n_ref: &i32 = &v[0];
            let n: i32 = *n_ref;
            println!("\nAfter assign *n_ref to n, n is {n}");
        }
        move_out_of_collection();

        // - below program does not work becase String is in heap
            // - and String does not have a Copy trait so *s_ref is trying to take ownership from v 
            // - but s_ref is a ref which is non-owning i.e., we cannot take ownership through a reference 
        // fn move_out_of_collection1() {
        //     let v: Vec<String> = vec![String::from("Hello world")];
        //     let s_ref: &String = &v[0];
        //     let s: String = *s_ref;
        //     println!("\nAfter assign *s_ref to n, n is {s}");
        // }
        // move_out_of_collection1();

        // - safe access to an element of collection
            // - immutable ref
        let v: Vec<String>= vec!["abc".into(), "def".into(), "ghi".into()];
        let s_ref = &v[0];
        println!("s_ref is immutable ref to {s_ref}");
            // - clone
        let mut s = v[0].clone();
        s.push('!'); 
        println!("s is mutable String:{s}");
            // - Vec::remove
        let mut v: Vec<String>= vec!["abc".into(), "def".into(), "ghi".into()];
        let mut s = v.remove(0);
        s.push('!'); 
        println!("s is mutable String:{s}");
        println!("After v.remove(), v is now: {:?}", v);
    
    // Fixing a Safe Program:Mutating Different Tuple Fields
        // - below safe_mutate() works
        fn safe_mutate() {
            let mut name: (String, String) = ("Ferris".into(), "Rustacean".into());
            // - this assignment borrows name.0 and name but not name.1
            let first = &name.0;
            // - we can mutate name.1
            name.1.push_str(", Esq.");
            println!("\nAfter name.1.push_str(), first is {first}, name.1 is {}", name.1);
        }
        safe_mutate();
        // - below safe_mutate() does not work
        fn safe_mutate1() {
            fn get_first(name: &(String, String)) -> &String {
                &name.0
            }
            let mut name: (String, String) = ("Ferris".into(), "Rustacean".into());
            // - Rust conservatively think this assignment borrows name.0, name ,and name.1
                // - Rust may change
            let _first = get_first(&mut name);
            // - so we can not mutate name.1
            // name.1.push_str(", Esq.");
            // println!("\nAfter name.1.push_str(), first is {first}, name.1 is {}", name.1);
        }
        safe_mutate1();

    // Fixing a Safe Program: Mutating Different Array Elements
        // - below program works
        fn mut_ref_arr() {
            let mut a = [0,1,2,3];
            let x = &mut a[1];
            *x += 1;
            println!("\nAfter *x += 1, a is now {a:?}");
        }
        mut_ref_arr();
        // - below program does not work
            // - Rust's borrow checker does not contain different paths for a[0], a[1], and so on. 
            // - it uses a single path a[_] that represents all indexes of a because Rust cannot always determine the value of an index

        // fn mut_ref_arr1() {
        //     let mut a = [0,1,2,3];
        //         // - this assignment borrows a[_] as mutable 
        //     let x = &mut a[1];
        //         // - this assignment tries to borrow again as immutable so is rejected by compiler though it is safe
        //     let y = &a[2];
        //     *x += *y;
        //     println!("\nAfter *x += *y, a is now {a:?}");
        // }

        // - workaround use standard library, e.g., slice::split_at_mut
        fn mut_ref_arr2() {
            let mut a = [0, 1, 2, 3];
            // - split_at_mut divides one mutable slice into two, at provided index
            let (a_l, a_r) = a.split_at_mut(2);
            let x = &mut a_l[1];
            let y = &a_r[0];
            *x += *y;
            println!("After *x += *y, a is now {a:?}");
        }
        mut_ref_arr2();
    }
}
