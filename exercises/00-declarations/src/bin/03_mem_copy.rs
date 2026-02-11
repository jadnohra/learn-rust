use spelled::explicit;

fn example() {
    let x = 5;
    explicit! {
        // Combination: let owner(y) = mem_copy(x)
        // Meaning: duplicate SPACE, x remains valid, y owns the copy
        //
        // Stack:
        // ┌─────────┐
        // │    5    │ <--- x (owner)
        // ├─────────┤
        // │    5    │ <--- y (owner of copy)
        // └─────────┘
        //
        // Both bindings own independent SPACE
        let owner(y) = mem_copy(x);
        println!("x = {}, y = {}", x, y);
    }
}

fn exercise() {
    let x = 5;
    // Write the real Rust equivalent of: let owner(y) = mem_copy(x)
    // let y = ??? x;
    // println!("x = {}, y = {}", x, y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
