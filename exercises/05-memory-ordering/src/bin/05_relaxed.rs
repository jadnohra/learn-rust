//! Exercise 5: Relaxed: When You Don't Need Visibility
//!
//! Relaxed is enough when you only need atomicity

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn example() {
    COUNTER.store(0, Ordering::SeqCst);

    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            for _ in 0..1000 {
                COUNTER.fetch_add(1, Ordering::Relaxed);
            }
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("Final: {}", COUNTER.load(Ordering::Relaxed));  // Always 10000

    // Why Relaxed works: we only care about the final count
    // Each increment is atomic (no lost updates)
}

fn exercise() {
    // TODO: Implement a counter where intermediate values matter
    //
    // For example: thread A increments, thread B reads and acts on value
    // Does Relaxed still work? When would you need stronger ordering?
    //
    // Question: What does "atomic" mean without ordering guarantees?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
