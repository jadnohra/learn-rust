//! Exercise 3: IDENTITY Validity (Lifetimes)
//!
//! A lifetime is not memory duration. It's IDENTITY validity in TIME.

fn example() {
    // Valid: reference lives within the scope of the data
    let x = 5;
    let r = &x;
    println!("r points to: {}", r);
    // r and x both end here - no problem

    // Also valid: pass reference to function
    fn use_ref(r: &i32) {
        println!("Function received: {}", r);
    }

    let y = 10;
    use_ref(&y);

    // Lifetimes ensure IDENTITY never outlives SPACE
}

fn exercise() {
    // TODO: Write a function that tries to return a reference to a local variable
    //
    // fn create_dangling() -> &i32 {
    //     let x = 5;        // SPACE created
    //     &x                // IDENTITY to that SPACE
    // }                     // SPACE ends. IDENTITY returned. Problem!
    //
    // Uncomment and observe the error.
    // What would happen if Rust allowed this?
    // The IDENTITY would outlive the SPACE it points to - a dangling reference.

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
