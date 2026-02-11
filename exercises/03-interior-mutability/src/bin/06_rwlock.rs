//! Exercise 6: RwLock: Many Readers or One Writer
//!
//! RwLock is like RefCell for threads

use std::sync::RwLock;

fn example() {
    let x = RwLock::new(5);

    // Multiple read locks OK
    {
        let r1 = x.read().unwrap();
        let r2 = x.read().unwrap();
        println!("r1 = {}, r2 = {}", *r1, *r2);
    }

    // Write lock needs exclusive access
    {
        let mut w = x.write().unwrap();
        *w = 10;
        println!("After write: {}", *w);
    }

    println!("Final: {}", *x.read().unwrap());

    // Map to: same as &T / &mut T, enforced at runtime across threads
}

fn exercise() {
    // TODO: Create an RwLock<Vec<i32>>
    // TODO: Spawn multiple reader threads
    // TODO: Have one writer thread modify the data
    //
    // Questions:
    // - What happens if a reader holds the lock while writer tries to write?
    // - Compare to Mutex: when would you prefer RwLock?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
