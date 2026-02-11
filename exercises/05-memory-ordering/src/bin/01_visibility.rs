//! Exercise 1: The Visibility Problem
//!
//! One thread writes, another reads. When is the write visible?

use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn example() {
    X.store(0, Ordering::SeqCst);  // Reset

    let writer = thread::spawn(|| {
        X.store(42, Ordering::Relaxed);
    });

    let reader = thread::spawn(|| {
        X.load(Ordering::Relaxed)
    });

    writer.join().unwrap();
    let value = reader.join().unwrap();

    println!("Read: {}", value);
    // Might be 0 or 42 - Relaxed gives no visibility guarantees
}

fn exercise() {
    // TODO: Run the example many times in a loop
    // TODO: Count how often you see 0 vs 42
    //
    // for _ in 0..1000 {
    //     // spawn writer and reader
    //     // record the result
    // }
    //
    // Map to: Relaxed gives no visibility guarantees between TIME lines
    // The result depends on scheduling and hardware

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
