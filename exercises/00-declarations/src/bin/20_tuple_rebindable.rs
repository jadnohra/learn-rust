use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let t = (1, 2);
    explicit! {
        // Combination: let (owner(rebindable(a)), owner(rebindable(b))) = t
        // Meaning: unpack, both own their values, both rebindable
        let (owner(rebindable(a)), owner(rebindable(b))) = t;
        a = take_or_mem_copy(100);
        b = take_or_mem_copy(200);
        println!("a = {}, b = {}", a, b);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let t = (1, 2);
    // Write the real Rust equivalent:
    // let (??? a, ??? b) = t;
    // a = 100;
    // b = 200;
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
