//! Exercise 9: When Compile-Time Is Too Conservative
//!
//! The borrow checker rejects some valid programs

fn example() {
    let mut v = vec![1, 2, 3, 4, 5];

    // This doesn't work - borrow checker can't prove disjoint:
    // let a = &mut v[0];
    // let b = &mut v[4];
    // *a = 10;
    // *b = 50;

    // Fix: use split_at_mut (uses unsafe internally)
    let (left, right) = v.split_at_mut(3);
    left[0] = 10;
    right[1] = 50;  // This is v[4]

    println!("v = {:?}", v);

    // split_at_mut does what the borrow checker can't prove
    // It uses unsafe inside to create two non-overlapping &mut
}

fn exercise() {
    // TODO: Try the "doesn't work" version - see the error
    //
    // let mut v = vec![1, 2, 3, 4, 5];
    // let a = &mut v[0];
    // let b = &mut v[4];  // What error do you get?
    //
    // Then fix it using split_at_mut or indices
    //
    // Question: Why can't the borrow checker prove v[0] and v[4] are disjoint?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
