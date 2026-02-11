//! Exercise 3: RefCell: Runtime Borrow Checker
//!
//! Same rule as compile-time, different verification TIME

use std::cell::RefCell;

fn example() {
    let x = RefCell::new(vec![1, 2, 3]);

    // Borrow shared twice - works
    {
        let r1 = x.borrow();
        let r2 = x.borrow();
        println!("r1 = {:?}, r2 = {:?}", *r1, *r2);
    }

    // Borrow mut - works when no other borrows
    {
        let mut m = x.borrow_mut();
        m.push(4);
        println!("After push: {:?}", *m);
    }

    // Same rule: !(shared IDENTITY && mutation)
}

fn exercise() {
    let x = RefCell::new(vec![1, 2, 3]);

    // TODO: Create a shared borrow
    // TODO: While it exists, try to create a mutable borrow
    //
    // let r = x.borrow();
    // let m = x.borrow_mut();  // What happens?
    //
    // Compare to compile-time: same rule, runtime panic instead of compile error

    let _ = x;
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
