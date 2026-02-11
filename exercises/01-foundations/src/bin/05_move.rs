//! Exercise 5: Representation Constraint: Can't Delete Values
//!
//! Move exists because you can't free a value mid-scope.

fn example() {
    let x = vec![1, 2, 3];
    println!("x = {:?}", x);

    let y = x;  // IDENTITY transferred (move)
    println!("y = {:?}", y);

    // x still exists as a name, but its IDENTITY is gone
    // println!("{:?}", x);  // Would error: "value borrowed after move"

    // Can we shadow x after the move? Yes!
    let x = 5;
    println!("x (shadowed) = {}", x);

    // The name 'x' persists. The IDENTITY was severed by the move.
    // Move simulates deletion by making the name unusable.
}

fn exercise() {
    // TODO: Create a String, move it to another binding, then shadow the original
    //
    // 1. let s = String::from("hello");
    // 2. Move s to a new binding
    // 3. Try to use the original s (see the error)
    // 4. Shadow s with something else
    //
    // What does this tell you about names vs IDENTITY?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
