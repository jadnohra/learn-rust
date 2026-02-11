//! Exercise 5: Mutex vs RefCell
//!
//! RefCell: single-thread, panics on violation
//! Mutex: multi-thread, blocks on violation

use std::cell::RefCell;
use std::sync::Mutex;

fn example() {
    let refcell = RefCell::new(5);
    let mutex = Mutex::new(5);

    // RefCell: immediate panic if rules violated
    println!("RefCell: {}", *refcell.borrow());

    // Mutex: blocks until lock available
    println!("Mutex: {}", *mutex.lock().unwrap());

    // Key difference: RefCell panics, Mutex blocks
}

fn exercise() {
    // TODO: Try to send RefCell to another thread
    //
    // use std::thread;
    // let refcell = RefCell::new(5);
    // let r = &refcell;
    // thread::spawn(move || { println!("{}", r.borrow()); });
    //
    // What error do you get?
    // Map to: RefCell is not Sync. Mutex is Sync.
    //
    // Question: Why is RefCell not thread-safe?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
