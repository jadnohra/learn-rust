//! Exercise 1: Cell: Copy In/Out, No References
//!
//! Cell: shared IDENTITY, but no IDENTITY into contents

use std::cell::Cell;

fn example() {
    let x = Cell::new(5);
    let r1 = &x;
    let r2 = &x;  // Multiple shared IDENTITY to Cell

    r1.set(10);   // Mutate through shared IDENTITY
    println!("After r1.set(10): {}", r2.get());

    r2.set(20);
    println!("After r2.set(20): {}", r1.get());

    // Cell only gives you .get() (copy out) and .set() (copy in)
    // No IDENTITY into contents = no aliasing problem
}

fn exercise() {
    // TODO: Create a Cell<i32>
    // TODO: Create multiple shared references to it
    // TODO: Mutate through different references
    //
    // Try to get a reference to the contents:
    // let inner: &i32 = ???  // Can you do this?
    //
    // Why can't you get a reference inside?
    // Map to: Cell avoids aliasing by only allowing copy in/out

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
