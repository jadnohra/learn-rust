use spelled::explicit;

fn example() {
    let x = 5;
    explicit! {
        // Combination: let name(r) = coord_shared(x)
        // Meaning: create shared coordinates to x's SPACE, fixed label
        //
        // Stack:
        // ┌─────────┐
        // │    5    │ <--- x
        // ├─────────┤
        // │ ptr  ---│---> (x's slot)  <--- r1 (shared)
        // ├─────────┤
        // │ ptr  ---│---> (x's slot)  <--- r2 (shared)
        // └─────────┘
        //
        // Multiple shared coordinates to same SPACE: OK
        let name(r1) = coord_shared(x);
        let name(r2) = coord_shared(x);  // OK: multiple shared coordinates allowed
        println!("r1 = {}, r2 = {}, x = {}", r1, r2, x);
    }
}

fn exercise() {
    let x = 5;
    // Write the real Rust equivalent:
    // let r1 = ??? x;
    // let r2 = ??? x;
    // println!("r1 = {}, r2 = {}, x = {}", r1, r2, x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
