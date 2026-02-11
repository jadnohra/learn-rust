use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let mut a = 5;
    let mut b = 10;
    explicit! {
        // Combination: let name(rebindable(r)) = coord_exclusive(y)
        // Meaning: exclusive coordinates, rebindable
        //
        // Before:                       After at(r)=100, r=coord_exclusive(b):
        // ┌─────────┐                   ┌─────────┐
        // │    5    │ <--- a            │   100   │ <--- a (mutated via r)
        // ├─────────┤                   ├─────────┤
        // │   10    │ <--- b            │   10    │ <--- b
        // ├─────────┤                   ├─────────┤
        // │ ptr --->│ (a) <-- r (excl)  │ ptr --->│ (b) <-- r (retargeted)
        // └─────────┘                   └─────────┘
        let name(rebindable(r)) = coord_exclusive(a);
        at(r) = 100;
        r = coord_exclusive(b);  // rebind to different SPACE
        at(r) = 200;
        println!("a = {}, b = {}", a, b);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let mut a = 5;
    let mut b = 10;
    // Write the real Rust equivalent:
    // let ??? r = ??? a;
    // *r = 100;
    // r = ??? b;
    // *r = 200;
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
