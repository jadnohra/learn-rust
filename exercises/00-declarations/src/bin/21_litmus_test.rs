use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let mut y = 5;
    let r = &mut y;
    explicit! {
        // The litmus test: Rust's most confusing declaration syntax
        //
        // Current Rust: let mut &mut x = r;
        //   - mut after let: binding can change
        //   - &mut in pattern: match a mutable reference, extract target
        //
        // What it actually does:
        //   - r must be &mut T
        //   - get value at coordinates
        //   - bind x to a copy of the target (x owns the copy)
        //   - x can be rebound later
        let owner(rebindable(x)) = mem_copy(at(r));
        x = take_or_mem_copy(100);
        println!("x = {}", x);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let mut y = 5;
    let r = &mut y;
    // Write the real Rust equivalent:
    // let ??? x = *r;
    // x = 100;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
