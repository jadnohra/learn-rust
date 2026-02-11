//! Exercise 7: Borrow Checker vs Value Tracking
//!
//! The borrow checker tracks IDENTITY, not values

fn example() {
    let mut v = vec![1, 2, 3];

    // Borrow checker can't prove [0] and [1] are different:
    // let r0 = &mut v[0];
    // let r1 = &mut v[1];  // ERROR!

    // Fix: use split_at_mut
    let (left, right) = v.split_at_mut(1);
    let r0 = &mut left[0];
    let r1 = &mut right[0];  // This is v[1]
    *r0 = 10;
    *r1 = 20;

    println!("v = {:?}", v);

    // Map to: decidability. Value tracking is undecidable in general.
}

fn exercise() {
    let mut v = vec![1, 2, 3, 4, 5];

    // TODO: Try the "doesn't work" version:
    // let r0 = &mut v[0];
    // let r1 = &mut v[4];
    //
    // What error do you get?
    // Fix it using split_at_mut.
    //
    // Question: Why can't the compiler prove v[0] and v[4] don't overlap?

    let _ = v;
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
