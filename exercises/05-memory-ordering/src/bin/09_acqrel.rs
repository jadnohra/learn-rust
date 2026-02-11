//! Exercise 9: AcqRel: Read-Modify-Write
//!
//! AcqRel: Acquire + Release in one operation

use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn example() {
    X.store(0, Ordering::SeqCst);

    // fetch_add is read-modify-write: reads, adds, stores
    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            let old = X.fetch_add(1, Ordering::AcqRel);
            println!("Thread {} saw {}, incremented to {}", i, old, old + 1);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("Final: {}", X.load(Ordering::Acquire));

    // AcqRel = Acquire on the read + Release on the write
}

fn exercise() {
    // TODO: When to use AcqRel vs SeqCst?
    //
    // AcqRel: pairwise ordering between specific threads
    // SeqCst: global ordering visible to all threads
    //
    // Try: implement a simple ticket lock using fetch_add
    // Does it need AcqRel or SeqCst?
    //
    // static TICKET: AtomicUsize = AtomicUsize::new(0);
    // static SERVING: AtomicUsize = AtomicUsize::new(0);

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
