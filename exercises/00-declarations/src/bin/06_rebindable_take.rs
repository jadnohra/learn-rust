use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let x = String::from("hello");
    explicit! {
        // Combination: let owner(rebindable(y)) = take(x)
        // Meaning: transfer ownership to y, rebindable
        let owner(rebindable(y)) = take(x);
        // String::from() creates new owned SPACE on heap
        y = take_or_mem_copy(String::from("world"));
        println!("y = {}", y);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let x = String::from("hello");
    // Write the real Rust equivalent:
    // let ??? y = ??? x;
    // y = String::from("world");
    // println!("y = {}", y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
