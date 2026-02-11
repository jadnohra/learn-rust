//! Exercise 10: Building a Shared Counter
//!
//! Design exercise: four versions of a counter

use std::cell::Cell;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};

fn example() {
    // Version 1: Single-thread, Cell (simplest)
    let counter1 = Cell::new(0u64);
    counter1.set(counter1.get() + 1);
    println!("Cell counter: {}", counter1.get());

    // Version 2: Single-thread, RefCell (if you need methods on the value)
    let counter2 = RefCell::new(0u64);
    *counter2.borrow_mut() += 1;
    println!("RefCell counter: {}", counter2.borrow());

    // Version 3: Multi-thread, Arc<Mutex<T>>
    let counter3 = Arc::new(Mutex::new(0u64));
    *counter3.lock().unwrap() += 1;
    println!("Arc<Mutex> counter: {}", counter3.lock().unwrap());

    // Version 4: Multi-thread, AtomicU64 (best for counters)
    let counter4 = AtomicU64::new(0);
    counter4.fetch_add(1, Ordering::Relaxed);
    println!("Atomic counter: {}", counter4.load(Ordering::Relaxed));

    // Map each to: IDENTITY sharing, TIME synchronization, Cost
}

fn exercise() {
    // TODO: Implement a shared counter that multiple threads can increment
    //
    // 1. Use Arc<AtomicU64>
    // 2. Spawn 10 threads
    // 3. Each thread increments 1000 times
    // 4. Print the final count (should be 10000)
    //
    // Questions:
    // - Why is AtomicU64 better than Arc<Mutex<u64>> for this?
    // - When would you prefer Arc<Mutex<T>>?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
