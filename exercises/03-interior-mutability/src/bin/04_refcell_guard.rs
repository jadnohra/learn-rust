//! Exercise 4: RefCell: The Guard Is The Borrow
//!
//! Ref and RefMut are guards. Their lifetime IS the borrow.

use std::cell::RefCell;

fn example() {
    let x = RefCell::new(5);

    let guard = x.borrow();    // Borrow starts
    println!("guard = {}", *guard);

    // Can't borrow_mut while guard exists
    // let m = x.borrow_mut();  // Would panic

    drop(guard);  // Borrow ends

    // Now borrow_mut works
    let mut m = x.borrow_mut();
    *m = 10;
    println!("After mutation: {}", *m);

    // Map to: the guard's existence = IDENTITY's TIME span
}

fn exercise() {
    let x = RefCell::new(vec![1, 2, 3]);

    // TODO: Demonstrate that dropping a guard ends the borrow
    //
    // 1. Take a borrow()
    // 2. Drop it explicitly
    // 3. Take a borrow_mut()
    // 4. Modify the vector
    //
    // Question: What happens if you forget to drop the shared borrow?

    let _ = x;
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
