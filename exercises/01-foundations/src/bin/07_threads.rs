//! Exercise 7: IDENTITY Across TIME Lines (Threads)
//!
//! Parallel TIME creates the coherence problem.

use std::thread;

fn example() {
    // This works: move ownership into the thread
    let x = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // x is now owned by this thread
        println!("Thread owns: {:?}", x);
    });

    handle.join().unwrap();

    // x is gone from main - ownership transferred
    // This is Rust's solution: don't share mutable IDENTITY across TIME lines

    // Alternative: use Arc for shared ownership
    use std::sync::Arc;
    let shared = Arc::new(vec![4, 5, 6]);
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        println!("Thread can read: {:?}", shared_clone);
    });

    handle.join().unwrap();
    println!("Main can still read: {:?}", shared);
}

fn exercise() {
    let mut x = 5;

    // TODO: Uncomment and observe the error:
    // let handle = thread::spawn(|| {
    //     x += 1;  // Try to mutate from another TIME line
    // });
    // handle.join().unwrap();
    //
    // What error do you get?
    // Map it to: IDENTITY crossing TIME line boundaries requires Send/Sync
    // The closure captures x, but:
    // - x might not live long enough (lifetime)
    // - x might be accessed from two TIME lines (data race)

    println!("x = {}", x);

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
