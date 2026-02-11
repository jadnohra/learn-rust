use spelled::explicit;

fn example() {
    explicit! {
        // Combination: let owner(x) = 5
        // Meaning: new SPACE with literal value, x owns it
        //
        // Stack:
        // ┌─────────┐
        // │    5    │ <--- x (owner)
        // └─────────┘
        let owner(x) = 5;
        println!("x = {}", x);
    }
}

fn exercise() {
    // Write the real Rust equivalent:
    // let ??? x = 5;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
