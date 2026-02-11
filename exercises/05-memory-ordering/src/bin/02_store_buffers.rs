//! Exercise 2: Store Buffers: Local SPACE Before Shared SPACE
//!
//! Writes go to local buffer before reaching shared memory

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;

static DATA: AtomicI32 = AtomicI32::new(0);
static FLAG: AtomicBool = AtomicBool::new(false);

fn example() {
    DATA.store(0, Ordering::SeqCst);
    FLAG.store(false, Ordering::SeqCst);

    let producer = thread::spawn(|| {
        DATA.store(42, Ordering::Relaxed);
        FLAG.store(true, Ordering::Relaxed);
    });

    let consumer = thread::spawn(|| {
        while !FLAG.load(Ordering::Relaxed) {
            std::hint::spin_loop();
        }
        DATA.load(Ordering::Relaxed)
    });

    producer.join().unwrap();
    let data = consumer.join().unwrap();

    println!("DATA = {}", data);
    // Might not be 42! FLAG may be visible before DATA (reordering)
}

fn exercise() {
    // TODO: Run many iterations and check for anomalies
    //
    // for i in 0..1000 {
    //     // reset DATA and FLAG
    //     // run producer/consumer
    //     if data != 42 {
    //         println!("Anomaly at iteration {}: {}", i, data);
    //     }
    // }
    //
    // Map to: store buffers and reordering can make FLAG visible before DATA
    // Fix: use Release/Acquire (next exercise)

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
