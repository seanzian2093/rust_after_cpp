/// # Ch13.1 - Closures, Anominous Functions that Capture their Environments
/// * Closure don't usually require you to annotate types of parameter or return value like `fn` function do.
///     * because they are not exposed to users
/// * Closures can capture values from their environment in three ways, which directly map to 3 ways a `fn` can take a parameter
///     * borrowing immutably
///     * borrowing mutably
///     * taking ownership
/// 
/// * Closure decide which of these to use based on what the body of the function does with the captured values
///     * use keyword `move` before parameter list to force taking ownership of values even body of funtion does not need
///         * mostly useful when passing a closure to a new thread to move data so it is owned by the new thread 
///         * new thread might outlive the thread from which it is spawn
///         * thread is discussed in more details in Ch16
/// * A closure body can do any of the following to captured values
///     * move them out of the closure
///     * mutata them
///     * neith move nor muetate
///     * capture nothing from the environment to begin with
/// * The way a closure captures and handles values from the environment affects which traits the closure implements
///     * and traits ar how functions and structs specify what kinds of closures they can use
///     * closures will automatically implement 1, 2, or all 3 of the `Fn` traits, in additive fashion
/// * Three `Fn` traits
///     * `FnOnce` applies to closures that can be called once
///         * all closures implement at least this trait 
///         * A closure that moves captured values out of its body will only implement `FnOnce` and none of others because it can only be called once
///     * `FnMut` applies to closures that don't move captured values out of their body but might mutate. 
///         * These closures can be called more than once
///     * `Fn` applies to closures that don't move captured values of their body and don't mutate them, including thoses capture nothing from environment
///         * these clousre can be called more than once without mutating their environment which is
///         * important in cases such as calling a closre multiple times concurrently
/// * Closures Must Name Captured Lifetimes
///     * in some cases, we need to tell Rust about lifetimes of its returned value
use std::thread;

#[derive(Debug)]
pub struct Closures{
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Blue,
    Red,
}

#[derive(Debug)]
#[allow(dead_code, unused_attributes)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // `user_preference` is `Some` or `None`
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        // count number of Blue and Red
        let mut num_red = 0;
        let mut num_blue= 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red 
        } else {
            ShirtColor::Blue
        }
    }
}

#[allow(dead_code, unused_variables)]
impl Closures{
    pub fn print(&self) {
        println!("\n======The note on closures======");

        let store = Inventory{
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!("\nThe user with preference {:?} gets {:?}", user_pref1, giveaway1);

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    // We can annotate closures to the extent that they look like a `fn` function
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // - v3 and v4 requires Rust to infer the type by evaluating the closures respectively
    let add_one_v3 = |x|             { x + 1 };
    let _res3 = add_one_v3(5);

    let add_one_v4 = |x|               x + 1  ;
    let _res4 = add_one_v4(5);

        // - once infered, the type is fixed, other types may not be accepted
    let example_closure = |x| x;
            // - e.g., after assignment of s, `expample_closure` accepts String 
    let s = example_closure(String::from("hello"));
            // - below is an error
    // let n = example_closure(5);

    // Capturing References or Moving Ownership
        let list = vec![1, 2, 3];
        println!("\nBefore defining closure list is: {:?}", list);
    
        // - borrowing immutably by immutable ref
        let only_borrows = || println!("From closure list is: {:?}", list);
    
        println!("Before calling closure list is: {:?}", list);
        only_borrows();
        println!("After calling closure list is: {:?}", list);

        // - borrowing mutably by mutable ref
        let mut list2 = vec![4, 5, 6];
        println!("\nBefore defining closure list2 is : {:?}", list2);
    
        let mut borrows_mutably = || list2.push(7);

        borrows_mutably();
        println!("After calling closure list2 is: {:?}", list2);


        let list3 = vec![8, 9, 10];
        println!("\nBefore defining closure list3 is: {:?}", list3);

        thread::spawn(move || println!("From thread list3 is: {:?}", list3))
            .join()
            .unwrap();

            // - list3 is unusable after ownership was taken
        // println!("After calling closure list3 is: {:?}", list3);
    
    // Three `Fn` traits
        let mut list_rec = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
        // - the closure provided to `sort_by_key`
            // - returns `width` field of a `Rectangle` struct for each instance in the array
            // - so it implements `FnMut`
        list_rec.sort_by_key(|r| r.width);
        println!("\nAfter sort, list_rec is: {:#?}", list_rec);

        
        // - the closure provided to `sort_by_key`
            // - takes ownership of `value` and moves it out of the environment
            // - so it implements `FnOnce`, i.e., can only be called once
            // - compiler does not allow it here where it will be called more than once 

        // let mut sort_operations = vec![];
        // let value = String::from("by key called");
        // list_rec.sort_by_key(|r| {
        //     sort_operations.push(value);
        //     r.width
        // });

        // - the closure provided to `sort_by_key`
            // - takes mutable ref to `num_sort_operations`
            // - so it implements `FnMut`, i.e. can be called more than once
        let mut num_sort_operations = 0;
        list_rec.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("\n{:#?}, sorted in {num_sort_operations} operations", list_rec);

        // - below function is rejected by compiler
            // - because the return value, a closure, may outlive s_ref then it causes a use-after-free issue
            // - we need to tell Rust that the returned closure must not live longer than s_ref, explicitly

        // fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
        //     move || s_ref.to_string()
        // }

            // - we do so by using lifetime parameter, i.e., we are telling Rust that
                // - s_ref lives for 'a
                // - adding `+ 'a` to return type's trait bounds indicates that returned closure must live no longer than 'a
        fn make_a_cloner<'a> (s_ref: &'a str) -> impl Fn() -> String + 'a {
            move || s_ref.to_string()
        }
                // - based on elision rules, this can be more concise 
                    // - adding only `'_` to indicate that returned closure depends on some lifetime
        fn make_a_cloner2(s_ref: &str) -> impl Fn() -> String + '_ {
            move || s_ref.to_string()
        }

    }
}

