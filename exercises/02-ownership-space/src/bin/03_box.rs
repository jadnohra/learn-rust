//! Exercise 3: Box: Unique IDENTITY to Heap SPACE
//!
//! Box<T>: unique IDENTITY to heap SPACE
//! Owner ends SPACE's TIME when dropped

fn example() {
    let b1 = Box::new(5);
    println!("b1 = {}", b1);

    let b2 = b1;  // IDENTITY transferred (move)
    // b1 is now invalid - the IDENTITY moved to b2

    println!("b2 = {}", b2);

    // Map to: unique IDENTITY means IDENTITY transfer on assignment
    // There's only ever one owner of the heap SPACE
}

fn exercise() {
    // TODO: Create a Box<String>
    // TODO: Move it to another binding
    // TODO: Try to use the original (see the error)
    // TODO: Verify only one owner exists at a time
    //
    // Bonus: What happens if you clone the Box instead of moving?
    // let b2 = b1.clone();

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
