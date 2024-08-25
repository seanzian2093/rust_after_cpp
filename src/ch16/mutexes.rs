//! # Ch16.3 - Share-State Concurrency
//!     * Channels, in any programming language, are similar to single ownership
//!     * Shared memory concurrency is like multiple ownership, i.e., multiple threads access same memory location at same time
//!     * Using Mutexes to Allow Access to Data from One Thread at a Time
//!         * mutex, i.e., mutual exclusion,allows only one thread to access some data at any given time
//!         * to access the data in a mutex, a thread must first signal that it wants to access by asking to acquire the mutex's lock
//!             * the lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data
//!         * You must attempt to acquire the lock before using the data
//!         * when done with the data that mutex guards, you must unlock the data so other threads can acquire the lock

//! # Ch16.4 - Extensible Concurrency with `Sync` and `Send` Traits
//!     * Rust itself has very few concurrency features, i.e. almost every concurrency feature discussed so far are from standard library
//!     * Two concurrency concepts are embedded in Rust, i.e., `std::marker` trait `Sync` and `Send`
//!     * `Send` marker trait indicates that ownership of values of type implementing `Send` can be transferred between threads
//!         * almost every Rust type is `Send`, exclusing `Rc<T>`, etc
//!         * almost all primitive types are `Send`, excluding raw pointers, discussed in Ch19
//!         * any type composed entirely of `Send` types is automatically marked as `Send`
//!     * `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be transferred from multiple threads
//!         * i.e., any type `T` is `Sync` if `&T` is `Send`
//!         * primitive types are all `Sync` and types composed entirely of types that are `Sync` are also `Sync`
//!         * `Sync` means thread-safe almost
//!     * Special cases
//!         * `Rc<T>` is neither `Send` nor `Sync`
//!         * `RefCell<T>` and related `Cell<T>` types are `Send` if `T: Send`, but not `Sync`
//!         * `Mutex<T>` is both `Send` and `Sync`
//!         * `MutexGuard<'a, T>`, that is returned by `Mutex::lock` is `Sync` if `T: Sync` but not `Send`

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
#[allow(unused)]
pub struct Mutexes{
}

#[allow(unused)]
impl Mutexes{
    pub fn print(&self) {
        println!("\n======The note on mutex======");
    // Create a mutex
        let m = Mutex::new(5);
        {
        // - use `.lock()` method to ask for lock, 
            // - would fail if another thread holding the lock panicked
            // - in such case no one would ever be able to get the lock so use `unwrap` to have this thread panic 
            // - return value is of `LockResult` type, after `unwrap` to a `MutexGuard`
            // - `MutexGuard` is a smart pointer which 
                // - implement `Deref` trait so we can use `*` operator to get the pointee
                // - a `Drop` trait that release the lock automatically when a `MutexGuard` goes out of scope
            let mut num = m.lock().unwrap();
        // - after successfully acqurired the lock,treat the return value, `num` in this case, as mutable reference to the data inside
            *num = 6;
        }
    
        println!("m = {:?}", m);

    // Sharing Mutex<T> Between Multiple Threads
        // - a `Mutex` obj that multiple threads will access and mutate
            // - a pure `Mutex` obj will be moved to closure so future use is not available
        // let counter = Mutex::new(0);
            // - a `Rc` over `Mutex` obj will enable multiple ownerships so future use would be available
                // - but Rust complains about unsafety with sending a `Rc` obj between threads
                // - because `Rc` does not implement `Send` trait
        // let counter = Rc::new(Mutex::new(0));
            // - finally a `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.
                // - a for atomic, see details in `std::sync::atomic`
                // - with performance penalty
            // - `Arc<T>` is still not safe for concurent situations if its data contains reference

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            // - create a smart pointer from another
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // - if `count` is simply a `Mutex` obj, below code will not compile because it was moved to the closure in `thread::spawn`
        println!("Result: {}", *counter.lock().unwrap());
    
    // Quiz
        // - `Arc<T>` is still not safe for concurent situations if its data contains reference
        // - so below code will not compile

    // let s = String::from("Hello world");
    // let a = Arc::new(&s);
    // let a2 = Arc::clone(&a);
    // let t = thread::spawn(move || a2.len());
    // let len = t.join().unwrap();
    // println!("{} {}", a, len);
    
    }
}