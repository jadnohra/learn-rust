use spelled::explicit;

fn example() {
    let mut y = 5;
    explicit! {
        // Combination: let name(r) = coord_exclusive(y)
        // Meaning: create exclusive coordinates to y's SPACE, mutation allowed, fixed label
        //
        // 1. Before borrow:
        // ┌─────────┐
        // │    5    │ <--- y
        // └─────────┘
        //
        // 2. During borrow (y suspended):
        // ┌─────────┐
        // │    5    │ <... y (suspended while r lives)
        // ├─────────┤
        // │ ptr --->│ (y's slot) <--- r (exclusive)
        // └─────────┘
        //
        // 3. After r drops (y restored):
        // ┌─────────┐
        // │   10    │ <--- y (usable again)
        // └─────────┘
        //
        // Key: borrow = temporary suspension, not permanent deletion like move
        let name(r) = coord_exclusive(y);
        at(r) = 10;  // mutate through exclusive coordinates
        println!("y = {}", y);
    }
}

fn exercise() {
    let mut y = 5;
    // Write the real Rust equivalent:
    // let r = ??? y;
    // *r = 10;
    // println!("y = {}", y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
