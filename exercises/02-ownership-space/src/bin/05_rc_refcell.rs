//! Exercise 5: Rc + RefCell: Shared IDENTITY + Mutation
//!
//! Rc handles SPACE x IDENTITY (who can access)
//! RefCell handles TIME (when mutation is safe)

use std::rc::Rc;
use std::cell::RefCell;

fn example() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    println!("Initial: {:?}", shared.borrow());

    // Mutate through a
    a.borrow_mut().push(4);
    println!("After a pushes 4: {:?}", shared.borrow());

    // Observe through b
    println!("b sees: {:?}", b.borrow());

    // Mutate through b
    b.borrow_mut().push(5);
    println!("After b pushes 5: {:?}", a.borrow());

    // Both IDENTITYs see the mutation.
    // RefCell ensures they don't conflict in TIME.
}

fn exercise() {
    // TODO: Create an Rc<RefCell<HashMap<String, i32>>>
    // TODO: Clone it to multiple owners
    // TODO: Insert values through different owners
    // TODO: Verify all owners see the same data
    //
    // Bonus: Try to hold two borrow_mut() at once - what happens?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
