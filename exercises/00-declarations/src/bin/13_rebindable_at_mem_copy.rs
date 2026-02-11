use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let x = 5;
    explicit! {
        let name(r) = coord_shared(x);
        // Combination: let owner(rebindable(y)) = mem_copy(at(r))
        // Meaning: get value at coordinates, duplicate, y owns it, rebindable
        let owner(rebindable(y)) = mem_copy(at(r));
        y = take_or_mem_copy(100);
        println!("x = {}, y = {}", x, y);  // x unchanged, y modified
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let x = 5;
    // Write the real Rust equivalent:
    // let r = ??? x;
    // let ??? y = *r;
    // y = 100;
    // println!("x = {}, y = {}", x, y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
