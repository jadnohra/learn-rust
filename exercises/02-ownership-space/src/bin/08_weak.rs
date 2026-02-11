//! Exercise 8: Weak: IDENTITY Without SPACE Ownership
//!
//! Weak<T>: IDENTITY that doesn't keep SPACE alive
//! Used to break cycles

use std::rc::{Rc, Weak};

fn example() {
    let strong = Rc::new(5);
    let weak: Weak<i32> = Rc::downgrade(&strong);

    println!("strong count: {}", Rc::strong_count(&strong));
    println!("weak count: {}", Rc::weak_count(&strong));

    // Access through weak (must upgrade to Option<Rc<T>>)
    if let Some(val) = weak.upgrade() {
        println!("Weak upgraded: {}", val);
    }

    // Drop strong
    drop(strong);

    // Try to access through weak again
    match weak.upgrade() {
        Some(val) => println!("Still alive: {}", val),
        None => println!("SPACE is gone, weak returns None"),
    }

    // Map to: Weak is observer IDENTITY, doesn't extend SPACE's TIME
}

fn exercise() {
    // TODO: Create a parent-child relationship where:
    // - Parent owns children (Rc<RefCell<Vec<Rc<Child>>>>)
    // - Children reference parent (Weak<Parent>)
    //
    // This prevents a reference cycle that would leak memory.
    //
    // Hint: Define structs like:
    // struct Parent { children: RefCell<Vec<Rc<Child>>> }
    // struct Child { parent: Weak<Parent> }

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
