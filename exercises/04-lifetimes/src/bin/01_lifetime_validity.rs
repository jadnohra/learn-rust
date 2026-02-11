//! Exercise 1: Lifetime = IDENTITY Validity
//!
//! 'a is not memory duration. It's how long IDENTITY is valid.

fn pass_through<'a>(x: &'a i32) -> &'a i32 {
    x  // Returned IDENTITY valid for same TIME span as input
}

fn example() {
    let outer = 5;
    let result;
    {
        let _inner = 10;
        result = pass_through(&outer);  // Works: outer's IDENTITY still valid

        // Can't do: result = pass_through(&inner);
        // inner's IDENTITY ends at block end
    }
    println!("result = {}", result);

    // Map to: IDENTITY can't outlive the SPACE it points to
}

fn exercise() {
    // TODO: Create a situation where a reference outlives its data
    //
    // let result;
    // {
    //     let inner = 10;
    //     result = &inner;  // What error do you get?
    // }
    // println!("{}", result);
    //
    // Map to: IDENTITY validity must not exceed SPACE duration

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
