//! Exercise 6: Arc: Shared IDENTITY Across Threads
//!
//! Rc: IDENTITY count not thread-safe
//! Arc: IDENTITY count is atomic (thread-safe)

use std::sync::Arc;
use std::thread;

fn example() {
    let data = Arc::new(vec![1, 2, 3]);

    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("Thread sees: {:?}", data_clone);
    });

    println!("Main sees: {:?}", data);
    handle.join().unwrap();

    // Arc allows IDENTITY to cross TIME line boundaries (Send)
}

fn exercise() {
    // TODO: Try the same with Rc instead of Arc (uncomment below)
    //
    // use std::rc::Rc;
    // let rc_data = Rc::new(vec![1, 2, 3]);
    // let rc_clone = Rc::clone(&rc_data);
    // thread::spawn(move || { println!("{:?}", rc_clone); });
    //
    // What error do you get?
    // Map to: Rc is not Send - can't cross thread boundaries
    // Arc uses atomic operations to be thread-safe

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
