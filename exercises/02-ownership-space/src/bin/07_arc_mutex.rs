//! Exercise 7: Arc + Mutex: Full Triangle
//!
//! Arc: shared IDENTITY across threads
//! Mutex: serialize TIME for mutation
//! Together: SPACE x TIME x IDENTITY all handled

use std::sync::{Arc, Mutex};
use std::thread;

fn example() {
    let data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..5).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut guard = data.lock().unwrap();
            *guard += 1;
            println!("Thread {} incremented to {}", i, *guard);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("Final: {}", *data.lock().unwrap());

    // Map each part:
    // Arc: who has IDENTITY (shared across threads)
    // Mutex: who can access in TIME (serialized)
    // The i32: the SPACE
}

fn exercise() {
    // TODO: Create an Arc<Mutex<Vec<i32>>>
    // TODO: Spawn 10 threads that each push their thread number
    // TODO: Print the final Vec
    //
    // Questions:
    // - What order do the numbers appear in?
    // - Is it deterministic? Run multiple times.
    // - What does this tell you about TIME serialization?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
