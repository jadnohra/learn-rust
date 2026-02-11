//! Exercise 2: Two Inputs, One Output: Whose Lifetime?
//!
//! When returning IDENTITY from multiple inputs, which TIME span applies?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn example() {
    let s1 = String::from("long string");
    {
        let s2 = String::from("short");
        let result = longest(&s1, &s2);
        println!("Longest: {}", result);  // Works inside block
    }
    // Can't use result here - s2 is gone

    // Map to: result's IDENTITY validity = intersection of input validities
}

fn exercise() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(&s1, &s2);
        println!("Inside: {}", result);
    }
    // TODO: Uncomment to see the error:
    // println!("Outside: {}", result);
    //
    // Why does this fail even though result might point to s1?
    // Map to: compiler doesn't know which branch was taken

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
