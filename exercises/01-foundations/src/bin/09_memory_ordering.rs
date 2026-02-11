//! Exercise 9: TIME Visibility (Memory Ordering)
//!
//! Hardware defers coherence. Orderings restore it.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;
use std::hint::black_box;

static DATA: AtomicI32 = AtomicI32::new(0);
static FLAG: AtomicBool = AtomicBool::new(false);

fn example() {
    // Reset for demo
    DATA.store(0, Ordering::SeqCst);
    FLAG.store(false, Ordering::SeqCst);

    // Producer: write DATA, then FLAG (with proper ordering)
    let producer = thread::spawn(|| {
        DATA.store(42, Ordering::Relaxed);
        FLAG.store(true, Ordering::Release);  // Release ensures DATA is visible
    });

    // Consumer: wait for FLAG, then read DATA (with proper ordering)
    let consumer = thread::spawn(|| {
        while !FLAG.load(Ordering::Acquire) {  // Acquire syncs with Release
            std::hint::spin_loop();
        }
        let data = DATA.load(Ordering::Relaxed);
        println!("DATA = {} (guaranteed to be 42 with Release/Acquire)", data);
        black_box(data);
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    // Release flushes local SPACE, Acquire syncs TIME
    // Without these orderings, DATA might not be visible yet!
}

fn exercise() {
    // Reset
    DATA.store(0, Ordering::SeqCst);
    FLAG.store(false, Ordering::SeqCst);

    // TODO: Modify this to use Relaxed ordering on both FLAG operations
    // TODO: Run multiple times - do you always see DATA = 42?
    //
    // With Relaxed: DATA might not be 42 (reordering, visibility)
    // With Release/Acquire: DATA guaranteed to be 42
    // Map to: Release flushes local SPACE, Acquire syncs TIME

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
