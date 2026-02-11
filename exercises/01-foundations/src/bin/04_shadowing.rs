//! Exercise 4: Representation Constraint: Can't Delete Names
//!
//! Shadowing exists because you can't undeclare a name mid-scope.

struct Droppable(&'static str);

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping: {}", self.0);
    }
}

fn example() {
    println!("=== Shadowing Demo ===");

    let x = Droppable("first x");
    println!("x allocated");

    let x = Droppable("second x (shadow)");
    println!("x shadowed - is the first one freed yet?");

    println!("About to exit scope...");

    // When is each Droppable actually freed?
    // Observe: shadowing hides, it doesn't delete.
    // The old SPACE still exists until scope end.
}

fn exercise() {
    println!("\n=== Your Turn ===");

    // TODO: Create a Droppable, shadow it twice more, and predict the drop order
    //
    // Before running, write down your prediction:
    // "I expect drops in this order: ___"
    //
    // Then run and verify. Explain why shadowing doesn't free the old SPACE.

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
