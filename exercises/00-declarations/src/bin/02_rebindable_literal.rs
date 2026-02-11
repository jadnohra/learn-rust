use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    explicit! {
        // Combination: let owner(rebindable(x)) = 5
        // Meaning: new SPACE, x owns it, rebindable
        //
        // Before:            After x = 10:
        // ┌─────────┐        ┌─────────┐
        // │    5    │ <-- x  │   10    │ <-- x  (same slot, new value)
        // └─────────┘        └─────────┘
        let owner(rebindable(x)) = 5;
        x = take_or_mem_copy(10);
        println!("x = {}", x);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    // Write the real Rust equivalent:
    // let ??? x = 5;
    // x = 10;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
