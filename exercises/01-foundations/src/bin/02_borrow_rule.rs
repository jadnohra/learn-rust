//! Exercise 2: The Borrow Checker Rule
//!
//! !(shared IDENTITY && mutation)

fn example() {
    let mut x = 5;

    // Exclusive access - mutation allowed
    let r = &mut x;
    *r = 10;
    println!("After exclusive mutation: x = {}", x);

    // Multiple shared access - observation only
    let r1 = &x;
    let r2 = &x;
    println!("Shared r1 = {}, r2 = {}", r1, r2);

    // The rule: you can have many &T OR one &mut T, never both simultaneously
}

fn exercise() {
    let mut x = 5;

    let r1 = &x;     // Shared IDENTITY
    let r2 = &x;     // Another shared IDENTITY

    // TODO: Try to mutate x here while r1 and r2 exist
    // TODO: Then try to use r1 and r2
    //
    // What does the error say? Map it to:
    // The rule: !(shared IDENTITY && mutation)

    println!("r1 = {}, r2 = {}", r1, r2);

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
