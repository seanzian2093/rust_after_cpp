// Ch4.2 - Reference and Borrow
#[derive(Debug)]
pub struct ReferenceBorrow{
}

impl ReferenceBorrow{
    pub fn print(&self) {
        println!("\n======The note of reference and borrow======");
        let m1 = String::from("Hello");
        let m2 = String::from("world");
        let (m1_again, m2_again) = greet(m1, m2);
        println!("After greet, m1_again is {m1_again}, m2_again is {m2_again}");

        greet2(&m1_again, &m2_again);
        println!("After greet2, m1_again is {m1_again}, m2_again is {m2_again}");
    
        // this greet takes ownership of g1 and g2, and return ownership
            // - this is inconvenient
        fn greet(g1: String, g2: String) -> (String, String) {
            println!("{} {}!", g1, g2);
            (g1, g2)
        }
    // References Are Non-Owning Pointers
        // - greet2 takes ref as parameter, does not take ownership of arguments
        fn greet2(g1: &String, g2: &String) {
            println!("{} {}!", g1, g2);
        }
    
    // Dereferencing a Pointer Accesses Its Data
    let mut x: Box<i32> = Box::new(1); // x points to a value in heap
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    println!("\na is now: {a}");
    *x += 1;                 // *x on the left-side modifies the heap value,
                                //     so x points to the value 2
    println!("x is now: {x}");
    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value
    println!("b is now: {b}");
    
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it
    println!("c is now: {c}");

        // - Rust implicitly inserts dereference and referecne in some cases
            // - multiple times if necessary
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    
    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);
    
    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
        
    // Rust Avoids Simultaneous Aliasing and Mutation
        // - data cannot be both aliased and mutated
        // - Rust enforces this principle for boxes(owned pointer, as opposed to ref which is non-owning pointer) by disallowing aliasing
            // - i.e., assign a box from one variable to another will move ownership, invalidating the previous variable
            // - owned data can only be accessed through owner, not aliases.
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
        // - push would resize v which deallocate the previous vector and allocate a new, bigger vector
            // - actually, such push is not allowed by compiler, error message is v is borrowed as immutable by num
                // - cannot be borrowed by push as mutable
    // v.push(4);
    println!("Third element through *num, is {}", *num);
    v.push(4);
        
    println!("After v.push fourth element is {}", &v[3]);

    // Mutable References Provide Unique and Non-Owning Access to Data
        // - num now is a mutable/unique reference, as opposed to immutable/shared reference
    let num: &mut i32 = &mut v[2];
        // - mutate through *num, not num itself whic is a mutable ref
    *num += 1;
        // - we can downgrade num
            // - num1 points to pointee too
            // - num lost write permission
    let num1 = &*num;
    println!("num1 points to {num1}, num points to {num}");
        // - if assign num to another variable, num will be unusable, i.e. lost all permission
    let num2 = num;
    println!("Num2 is now a mutable ref to v[2]. Third element is {}", *num2);
        // - mutate through v is not available, i.e. path v is unusable
    // v.push(4);

    // println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);

    // Permissions Are Returned At the End of a Reference's Lifetime
        // - from creation to last use
    }
}
