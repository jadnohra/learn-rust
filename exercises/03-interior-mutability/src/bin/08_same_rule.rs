//! Exercise 8: Interior Mutability: Same Rule, Different TIME
//!
//! Prove that RefCell enforces the same rule as the borrow checker

use std::cell::RefCell;

fn example() {
    // Runtime equivalent with RefCell:
    let x = RefCell::new(5);

    let r = x.borrow();
    println!("Shared borrow: {}", *r);

    // Can't borrow_mut while r exists (would panic)
    // let m = x.borrow_mut();  // PANIC at runtime

    drop(r);

    let m = x.borrow_mut();
    println!("Exclusive borrow: {}", *m);

    // They reject the same pattern. Only the TIME differs.
}

fn exercise() {
    // TODO: Write both versions side-by-side
    //
    // Compile-time version (this won't compile):
    // let mut x = 5;
    // let r = &x;
    // let m = &mut x;  // ERROR at compile time
    //
    // Runtime version:
    // let x = RefCell::new(5);
    // let r = x.borrow();
    // let m = x.borrow_mut();  // PANIC at runtime
    //
    // Verify: same rule, different enforcement TIME

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
