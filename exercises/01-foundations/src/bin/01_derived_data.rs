//! Exercise 1: Derived Data at Language Level
//!
//! Physics creates distance. Distance forces copies. Copies require coherence.

fn example() {
    // Rust prevents the coherence problem at compile time
    let mut x = 5;

    // This works: exclusive access, then observe
    let r = &mut x;
    *r = 10;
    // r's lifetime ends here

    println!("x = {}", x);  // Now we can observe

    // The rule: you cannot have shared IDENTITY while mutation is possible
}

fn exercise() {
    let mut x = 5;

    // TODO: Create two paths to x (two references)
    // TODO: Mutate through one path
    // TODO: Observe through the other
    //
    // Does Rust allow this? Why or why not?
    // Hint: Try creating two mutable references, or one mutable and one shared.
    // Map the error to: shared IDENTITY + mutation = coherence problem

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
