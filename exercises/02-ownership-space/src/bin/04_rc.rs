//! Exercise 4: Rc: Shared IDENTITY, Counted SPACE Lifetime
//!
//! Rc<T>: multiple IDENTITYs to same SPACE
//! SPACE's TIME ends when last IDENTITY drops

use std::rc::Rc;

fn example() {
    let a = Rc::new(5);
    println!("Created a, count: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);  // New IDENTITY, same SPACE
    println!("Cloned to b, count: {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("Cloned to c, count: {}", Rc::strong_count(&a));
    }  // c dropped here

    println!("c dropped, count: {}", Rc::strong_count(&a));

    drop(b);
    println!("b dropped, count: {}", Rc::strong_count(&a));

    // When a drops, count reaches 0, SPACE freed
    // Map to: counting IDENTITYs to decide SPACE lifetime
}

fn exercise() {
    // TODO: Create an Rc<Vec<i32>>
    // TODO: Clone it multiple times
    // TODO: Print the strong_count at each step
    // TODO: Drop some clones and observe the count decrease
    //
    // Questions:
    // - When is the Vec actually freed?
    // - Can you mutate through Rc? (Try it!)

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
