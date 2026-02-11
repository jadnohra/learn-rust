//! Exercise 6: NLL: IDENTITY Ends at Last Use
//!
//! Non-Lexical Lifetimes: IDENTITY validity ends at last use, not scope end

fn example() {
    let mut x = 5;

    let r = &x;          // Shared IDENTITY starts
    println!("{}", r);   // Last use of r
    // Shared IDENTITY ends here (NLL)

    let m = &mut x;      // Exclusive IDENTITY starts - OK!
    *m = 10;
    println!("{}", m);

    // Map to: TIME span = first use to last use, not lexical scope
}

fn exercise() {
    let mut x = 5;

    // TODO: Move the println!(r) after the &mut x. What happens?
    //
    // let r = &x;
    // let m = &mut x;
    // println!("{}", r);  // Error: shared borrow still in use
    //
    // The borrow checker sees r is used after m is created.
    // That violates: !(shared IDENTITY && mutation)

    let _ = x;
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
