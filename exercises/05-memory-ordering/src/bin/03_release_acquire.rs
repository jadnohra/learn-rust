//! Exercise 3: Release/Acquire: Sync Points
//!
//! Release: flush writes. Acquire: see writes before a Release.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;

static DATA: AtomicI32 = AtomicI32::new(0);
static FLAG: AtomicBool = AtomicBool::new(false);

fn example() {
    DATA.store(0, Ordering::SeqCst);
    FLAG.store(false, Ordering::SeqCst);

    let producer = thread::spawn(|| {
        DATA.store(42, Ordering::Relaxed);
        FLAG.store(true, Ordering::Release);  // Flush all writes
    });

    let consumer = thread::spawn(|| {
        while !FLAG.load(Ordering::Acquire) {  // Sync point
            std::hint::spin_loop();
        }
        let data = DATA.load(Ordering::Relaxed);  // Guaranteed 42
        println!("DATA = {}", data);
        assert_eq!(data, 42);
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    // Map to: happens-before edge between TIME lines
}

fn exercise() {
    // TODO: Verify that Release/Acquire fixes the visibility problem
    //
    // Run 1000 iterations with Release/Acquire
    // Verify DATA is always 42
    //
    // Then change back to Relaxed and see if anomalies return
    //
    // Question: Why can DATA.load be Relaxed after the Acquire?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
