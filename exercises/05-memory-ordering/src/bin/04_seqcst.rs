//! Exercise 4: SeqCst: Total Order
//!
//! All threads see the same order of all SeqCst operations

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

fn example() {
    A.store(false, Ordering::SeqCst);
    B.store(false, Ordering::SeqCst);

    let t1 = thread::spawn(|| {
        A.store(true, Ordering::SeqCst);
    });

    let t2 = thread::spawn(|| {
        B.store(true, Ordering::SeqCst);
    });

    let t3 = thread::spawn(|| {
        let a = A.load(Ordering::SeqCst);
        let b = B.load(Ordering::SeqCst);
        (a, b)
    });

    let t4 = thread::spawn(|| {
        let b = B.load(Ordering::SeqCst);
        let a = A.load(Ordering::SeqCst);
        (b, a)
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let r3 = t3.join().unwrap();
    let r4 = t4.join().unwrap();

    println!("T3 saw: A={}, B={}", r3.0, r3.1);
    println!("T4 saw: B={}, A={}", r4.0, r4.1);

    // With SeqCst: if T3 sees (true, false), T4 cannot see (true, false)
}

fn exercise() {
    // TODO: Change SeqCst to Acquire/Release and run many times
    // TODO: Can you observe both threads seeing (true, false)?
    //
    // SeqCst provides a total order all threads agree on
    // Acquire/Release only provides pairwise ordering
    //
    // Question: When do you need SeqCst vs Acquire/Release?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
