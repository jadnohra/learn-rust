//! Exercise 7: The Spectrum: Compile to Runtime to Unsafe
//!
//! Move along the spectrum. Observe the tradeoffs.

use std::cell::{Cell, RefCell, UnsafeCell};

fn example() {
    // Compile-time: borrow checker
    let mut x = 5;
    let r = &mut x;
    *r = 10;
    println!("Compile-time checked: {}", x);

    // Runtime: Cell (cheap, Copy only)
    let y = Cell::new(5);
    y.set(10);
    println!("Cell (copy in/out): {}", y.get());

    // Runtime: RefCell (flexible, can panic)
    let z = RefCell::new(5);
    *z.borrow_mut() = 10;
    println!("RefCell (runtime borrow check): {}", z.borrow());

    // Unsafe: UnsafeCell (no checks)
    let w = UnsafeCell::new(5);
    unsafe { *w.get() = 10; }
    println!("UnsafeCell (you prove it): {}", unsafe { *w.get() });

    // What do you gain at each step? More flexibility.
    // What do you lose? Compile-time guarantees.
}

fn exercise() {
    // TODO: For each level of the spectrum, identify:
    // 1. When you would use it
    // 2. What guarantees you have
    // 3. What can go wrong
    //
    // Fill in a mental table:
    // | Type       | When to use          | Guarantee          | Risk           |
    // |------------|----------------------|--------------------|----------------|
    // | &mut T     | ???                  | Compile-time safe  | ???            |
    // | Cell<T>    | ???                  | ???                | ???            |
    // | RefCell<T> | ???                  | ???                | Runtime panic  |
    // | UnsafeCell | ???                  | None               | ???            |

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
