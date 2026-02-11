//! Exercise 2: Cell Limitation: Must Be Copy
//!
//! Cell only works for Copy types

use std::cell::Cell;

fn example() {
    let x = Cell::new(5);      // i32 is Copy: works
    println!("x = {}", x.get());

    x.set(10);
    println!("After set: {}", x.get());

    // Cell works by copying values in and out
    // This only works for Copy types
}

fn exercise() {
    // TODO: Try to create a Cell<Vec<i32>> and call .get()
    //
    // let y = Cell::new(vec![1, 2, 3]);
    // let v = y.get();  // What error do you get?
    //
    // Map to: Cell avoids aliasing by copying. Can't copy non-Copy.
    // For non-Copy types, use RefCell instead.
    //
    // Question: What method CAN you use on Cell<Vec<i32>>?
    // Hint: try .take() or .replace()

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
