//! Exercise 8: The Borrow Checker Is Conservative
//!
//! It rejects some valid programs

fn example() {
    let mut x = 5;
    let r = &mut x;

    if false {
        println!("{}", r);  // Never executes
    }

    // Can't do: let s = &x;  // r might be used

    drop(r);  // Explicitly end r's lifetime

    let s = &x;  // Now OK
    println!("{}", s);

    // The borrow checker doesn't know `if false` never runs
    // Sound but incomplete: no false negatives, some false positives
}

fn exercise() {
    let mut x = 5;
    let r = &mut x;

    // TODO: Without using drop(r), try to take a shared borrow
    //
    // Hint: NLL means if you don't use r anymore, you can borrow again
    //
    // let r = &mut x;
    // // don't use r here
    // let s = &x;  // Does this work?
    //
    // Map to: NLL makes the borrow checker less conservative

    let _ = r;
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
