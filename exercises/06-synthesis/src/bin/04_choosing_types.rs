//! Exercise 4: Interior Mutability: Choosing the Right Type

use std::cell::{Cell, RefCell, OnceCell};
use std::sync::{Arc, Mutex, OnceLock};

fn example() {
    // Scenario 1: Counter incremented by callback, single thread
    let counter1 = Cell::new(0u64);
    let callback = || counter1.set(counter1.get() + 1);
    callback();
    callback();
    println!("Cell counter: {}", counter1.get());

    // Scenario 2: Cache that might compute on miss, single thread
    let cache: OnceCell<String> = OnceCell::new();
    let value = cache.get_or_init(|| {
        println!("Computing expensive value...");
        String::from("computed")
    });
    println!("Cached: {}", value);
    let _ = cache.get_or_init(|| panic!("Should not run"));

    // Scenario 3: Shared mutable state across threads
    let shared = Arc::new(Mutex::new(0));
    *shared.lock().unwrap() += 1;
    println!("Mutex: {}", shared.lock().unwrap());

    // Scenario 4: Write-once initialization, multi-thread
    static CONFIG: OnceLock<String> = OnceLock::new();
    CONFIG.get_or_init(|| String::from("initialized"));
    println!("OnceLock: {}", CONFIG.get().unwrap());
}

fn exercise() {
    // TODO: For each scenario, explain WHY that type was chosen
    //
    // Fill in the decision matrix:
    // | Scenario              | Type     | Single/Multi | Copy? | Mutable? |
    // |-----------------------|----------|--------------|-------|----------|
    // | Callback counter      | Cell     | Single       | Yes   | Yes      |
    // | Lazy cache            | OnceCell | Single       | No    | Once     |
    // | Shared counter        | ???      | Multi        | ???   | ???      |
    // | Global init           | ???      | Multi        | ???   | ???      |
    //
    // Then implement: a RefCell-based cache that can be updated (not just once)
    //
    // let cache: RefCell<Option<String>> = RefCell::new(None);

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
