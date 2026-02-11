use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let x = 5;
    explicit! {
        // Combination: let owner(rebindable(y)) = mem_copy(x)
        // Meaning: duplicate SPACE, y owns it, rebindable
        let owner(rebindable(y)) = mem_copy(x);
        y = take_or_mem_copy(20);
        println!("x = {}, y = {}", x, y);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let x = 5;
    // Write the real Rust equivalent:
    // let ??? y = ??? x;
    // y = 20;
    // println!("x = {}, y = {}", x, y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
