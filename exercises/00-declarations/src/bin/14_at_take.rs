use spelled::explicit;

fn example() {
    let b = Box::new(String::from("hello"));
    explicit! {
        // Combination: let owner(x) = take(at(b))
        // Meaning: get value at coordinates (Box), transfer ownership to x
        //
        // Before take(at(b)):
        //   Stack                  Heap                    Heap
        // ┌─────────┐        ┌─────────────┐         ┌─────────┐
        // │ ptr ----│------->│ ptr/len/cap │-------->│ "hello" │
        // └─────────┘ <-- b  └─────────────┘         └─────────┘
        //   (Box owner)        (String)
        //
        // After take(at(b)):
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "hello" │
        // └─────────────┘ <-- x (owner)        └─────────┘
        //                 <... b (invalid)
        // x owns String directly (same heap data)
        let owner(x) = take(at(b));
        println!("x = {}", x);
        // println!("b = {:?}", b);  // won't compile: b moved
    }
}

fn exercise() {
    let b = Box::new(String::from("hello"));
    // Write the real Rust equivalent:
    // Hint: You can move out of a Box by dereferencing it
    // let x = *b;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
