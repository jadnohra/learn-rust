//! Exercise 9: Drop Order: When Does SPACE End?
//!
//! SPACE ends when owner drops. But in what order?

struct Named(&'static str);

impl Drop for Named {
    fn drop(&mut self) {
        println!("Dropping: {}", self.0);
    }
}

fn example() {
    println!("Creating a, b, c...");
    let a = Named("a");
    let b = Named("b");
    let c = Named("c");

    println!("Creating tuple (x, y)...");
    let (x, y) = (Named("x"), Named("y"));

    println!("About to exit scope...");

    // Observe: SPACE ends in reverse declaration order (LIFO)
    // Tuple fields drop in order (x before y)
}

fn exercise() {
    println!("\n=== Your Turn ===");

    // TODO: Create a struct with multiple Named fields
    // TODO: Predict the drop order before running
    // TODO: Verify your prediction
    //
    // struct Container {
    //     first: Named,
    //     second: Named,
    //     third: Named,
    // }
    //
    // Questions:
    // - Do struct fields drop in declaration order or reverse?
    // - What about nested structs?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
