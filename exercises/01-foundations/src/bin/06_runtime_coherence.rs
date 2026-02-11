//! Exercise 6: Coherence Deferred to Runtime
//!
//! Same rule, different verification TIME.

use std::cell::RefCell;

fn example() {
    let data = RefCell::new(5);

    // Multiple shared borrows work
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("Shared: r1 = {}, r2 = {}", *r1, *r2);

    // Must drop shared borrows before getting exclusive
    drop(r1);
    drop(r2);

    // Now exclusive borrow works
    let mut m = data.borrow_mut();
    *m = 10;
    println!("After exclusive mutation: {}", *m);
    drop(m);

    println!("Final value: {}", *data.borrow());

    // Same rule as compile-time: !(shared IDENTITY && mutation)
    // Different TIME of verification: runtime instead of compile-time
}

fn exercise() {
    let data = RefCell::new(5);

    let r1 = data.borrow();      // Shared IDENTITY
    let r2 = data.borrow();      // Another shared IDENTITY

    println!("r1 = {}, r2 = {}", *r1, *r2);

    // TODO: Uncomment the next line to see a runtime panic:
    // let m = data.borrow_mut();  // Try to get exclusive while shared exist
    //
    // What happens? Why is this checked at runtime instead of compile time?
    // Compare: the borrow checker prevents this at compile time with regular &/&mut

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
