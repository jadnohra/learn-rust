use spelled::explicit;

fn example() {
    let x = String::from("hello");
    explicit! {
        // Combination: let owner(y) = take(x)
        // Meaning: transfer ownership, x invalidated, y now owns
        //
        // Before take(x):
        //      Stack                     Heap
        // ┌───────────────┐        ┌─────────────┐
        // │ ptr ----------│------->│ "hello"     │
        // │ len: 5        │ <-- x  └─────────────┘
        // │ cap: 5        │    (owner)
        // └───────────────┘
        //
        // After take(x):
        //      Stack                     Heap
        // ┌───────────────┐        ┌─────────────┐
        // │ ptr ----------│------->│ "hello"     │  (same heap)
        // │ len: 5        │ <-- y  └─────────────┘
        // │ cap: 5        │    (owner) <... x (invalid)
        // └───────────────┘
        let owner(y) = take(x);
        println!("y = {}", y);
        // println!("x = {}", x);  // won't compile: x moved
    }
}

fn exercise() {
    let x = String::from("hello");
    // Write the real Rust equivalent:
    // let y = ??? x;
    // println!("y = {}", y);
    // // println!("x = {}", x);  // won't compile: x moved
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
