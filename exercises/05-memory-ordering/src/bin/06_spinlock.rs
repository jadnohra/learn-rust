//! Exercise 6: Implementing a Spinlock
//!
//! A spinlock serializes TIME using atomics

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static LOCK: AtomicBool = AtomicBool::new(false);
static mut DATA: i32 = 0;

fn with_lock<F: FnOnce()>(f: F) {
    // Acquire lock
    while LOCK.compare_exchange(
        false, true,
        Ordering::Acquire,   // See writes from previous Release
        Ordering::Relaxed    // Failed CAS doesn't need ordering
    ).is_err() {
        std::hint::spin_loop();
    }

    f();

    // Release lock
    LOCK.store(false, Ordering::Release);  // Publish our writes
}

fn example() {
    unsafe { DATA = 0; }
    LOCK.store(false, Ordering::SeqCst);

    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            for _ in 0..1000 {
                with_lock(|| unsafe { DATA += 1; });
            }
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    unsafe { println!("DATA = {}", DATA); }  // Always 10000
}

fn exercise() {
    // TODO: What happens if you change Acquire to Relaxed in the lock?
    // TODO: What happens if you change Release to Relaxed in the unlock?
    //
    // Try it and reason about why it breaks (or doesn't)
    //
    // Map to: Acquire on lock sees previous critical section's writes
    //         Release on unlock publishes this critical section's writes

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
