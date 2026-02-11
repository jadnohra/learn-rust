//! Exercise 3: Lifetime Elision: What the Compiler Infers
//!
//! These are the same:

fn explicit<'a>(x: &'a str) -> &'a str { x }
fn elided(x: &str) -> &str { x }

fn example() {
    let s = String::from("hello");

    println!("explicit: {}", explicit(&s));
    println!("elided: {}", elided(&s));

    // Elision rules:
    // 1. Each input reference gets its own lifetime
    // 2. If one input, output gets that lifetime
    // 3. If &self, output gets self's lifetime
}

fn exercise() {
    // TODO: Write a function where elision doesn't work
    //
    // fn broken(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }
    //
    // What error do you get? Fix it by adding explicit lifetimes.
    // Why can't elision figure this out?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
