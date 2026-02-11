//! Exercise 8: Serializing TIME (Mutex)
//!
//! Mutex makes parallel TIME sequential.

use std::sync::{Arc, Mutex};
use std::thread;

fn example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented, value now {}", i, *num);
            // Lock released when `num` drops
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final: {}", *counter.lock().unwrap());

    // Observe: lock() serializes TIME
    // Only one thread accesses SPACE at a time
    // Arc handles shared IDENTITY across threads
    // Mutex handles TIME synchronization
}

fn exercise() {
    // TODO: Create your own Arc<Mutex<_>> counter
    // TODO: Spawn 10 threads that each increment it
    // TODO: Print the final value
    //
    // Questions:
    // - What guarantees does Mutex provide?
    // - What does Arc provide that Rc doesn't?
    // - Why can't you use Rc<Mutex<_>> across threads?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
