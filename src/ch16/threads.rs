//! # Ch16.1 - Using Threads to Run Code Simultaneously
//! ## Rust standard library uses 1:1 model of thread implementation
//!     * i.e., a program uses one operating syytem thread per one language thread
//!     * there are crates that implement other models of threading
use std::thread;
use std::time::Duration;
        
#[derive(Debug)]
#[allow(unused)]
pub struct Threads{
}

#[allow(unused)]
impl Threads{
    pub fn print(&self) {
        println!("\n======The note on threads======");
    // Creating a New Thread with `std::thread::spawn`
        // - takes a closure containing the code we want to run in the new thread
        // - caveats
            // - main threading ending stops the spawned thread, prematurely sometime
            // - no guarantee on the order in which threads run, or run at all 
        // - i.e., run in new thread
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // - run in main thread
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    // Waiting for All Threads to Finish Using `join` Handles
        // - return value of `thread::spawn` is a `JoinHandle` type
        // - when we call `join` method on it, it will wait for its thread to finish
            // - by blocking the thread that is currently running, i.e., preventing it from performing or exiting 
        println!("");
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    
        // - `join()` here is blocking main threading from performing work or exiting until the thread represented by `handle` finishes
        handle.join().unwrap();

    // Using `move` Closures with Threads
        // - closures take ownership of values from environment
        // - providing a closure to thread transfers owership of those values from one thread to another
        // - in some cases, we need to take ownership of those values. e.g.,
        let v = vec![1, 2, 3];
            // - the `println!` only needs a ref to v but there is no guarantee that v outlives the thread
                // - v could be dropped before the thread so compile resuses to compile and suggest we `move` v
        // let handle = thread::spawn(|| {
        //     println!("Here's a vector: {:?}", v);
        // });

        let handle = thread::spawn(move || {
            println!("\nHere's a vector taken from main thread: {:?}", v);
        });

        handle.join().unwrap();

    // Quiz
        // - when i32 is taken ownership, its content is copied so in this case, n will be 2 at the end 
    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        thread::spawn(move || {
            n = n + 1;
        })
    });
    n = n + 1;
    t.join().unwrap().join().unwrap();
    println!("{n}");

    }
}
