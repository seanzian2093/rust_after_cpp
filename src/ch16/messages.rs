//! # Ch16.2 - Using Message Passing to Transfer Data Between Threads

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
#[allow(unused)]
pub struct Messages {}

#[allow(unused)]
impl Messages {
    pub fn print(&self) {
        println!("\n======The note on messages between threads======");
        // Create a channel using `mpsc::channel` function
        // - `mpsc` stands for multiple producer, single consumer,i.e.,
        // - a channel can have multiple sending ends/transmitter but only one receiving end/receiver
        // - `mpsc::channel` return a tuple(sending end, receiving end)
        let (tx, rx) = mpsc::channel();

        // Moving a transmitter, `tx` to a spawned thread and sending "hi"
        thread::spawn(move || {
            let val = String::from("hi");
            println!("\nSending: {} from a spawned thread", val);
            tx.send(val).unwrap();
            // - after sending, val is unusable in this scope so below code does not compile
            // println!("After sending, val is {} in this spawned thread", val);
        });

        // Receiving the value in main thread and use it
        // - `.recv()` method blocks the currently running thread until a value is sent down the channel
        // - returns a `Result<T, E>` type
        // - when the transmitter closes, `recv` will return an error to signal that no more values will be coming
        // - `.try_recv` does not block and immediately returns a `Result<T, E>`
        // - an `Ok` holding the value/message received, an `Error` if there is not any this time.
        // - we could write loop of `try_recv` to check message
        let received = rx.recv().unwrap();
        println!("Got: {} in main thread", received);

        // Sending Multiple Values and Seeing the Receiver Waiting
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // - using receiver `rx` as iterator, not calling `recv` method
        // - main thread wait for each value sent from spawned thread
        println!("");
        for received in rx {
            println!("Got: {}", received);
        }

        // Creating Multiple Producers by Cloning the Transmitter
        let (tx, rx) = mpsc::channel();

        // - #1 transmitter
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // - #2 transmitter
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("");
        for received in rx {
            println!("Got: {}", received);
        }
    }
}
