use spelled::explicit;

fn example() {
    let x1 = 5;                        // i32: Copy
    let x2 = String::from("hello");    // String: Move
    explicit! {
        // Combination: let owner(y) = take_or_mem_copy(x)
        // Meaning: compiler decides based on type, y owns result
        //
        // i32 (Copy):               String (Move):
        // ┌─────────┐               ┌───────────┐        ┌─────────┐
        // │    5    │ <-- x1        │ ptr ------│------->│ "hello" │
        // ├─────────┤    (owner)    │ len: 5    │ <-- y2 └─────────┘
        // │    5    │ <-- y1        │ cap: 5    │    (owner) <... x2 (invalid)
        // └─────────┘    (owner)    └───────────┘
        // x1 still valid            (same heap, moved)
        let owner(y1) = take_or_mem_copy(x1);
        let owner(y2) = take_or_mem_copy(x2);

        println!("x1 = {}, y1 = {}", x1, y1);  // x1 still valid (was copied)
        println!("y2 = {}", y2);
        // println!("x2 = {}", x2);  // won't compile: x2 moved
    }
}

fn exercise() {
    let x1 = 5;                        // i32: Copy
    let x2 = String::from("hello");    // String: Move
    // Write the real Rust equivalent:
    // Hint: real Rust uses the same syntax for both—the type determines behavior
    // let y1 = x1;
    // let y2 = x2;
    // println!("x1 = {}, y1 = {}", x1, y1);
    // println!("y2 = {}", y2);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
