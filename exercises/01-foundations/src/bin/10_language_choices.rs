//! Exercise 10: Languages Are Choices
//!
//! Different languages constrain different axes.

fn example() {
    // Rust constrains IDENTITY (ownership, borrowing)
    rust_way();

    // What if we constrained TIME instead? (immutability)
    immutable_way();
}

fn rust_way() {
    let mut x = 5;
    let r = &mut x;  // Exclusive IDENTITY
    *r = 10;         // Mutation via unique path
    println!("Rust way: x = {}", x);

    // Rust's choice: control IDENTITY strictly, allow TIME to flow
}

fn immutable_way() {
    let x = 5;
    let x = x + 5;  // New SPACE, new IDENTITY, old unchanged
    println!("Immutable way: x = {}", x);

    // This is "freezing TIME" - no mutation, only new values
    // Haskell, Clojure, Erlang take this path
    // Trade: more allocations, but simpler reasoning about IDENTITY
}

fn exercise() {
    // TODO: Implement a third approach - message passing style
    //
    // Instead of sharing mutable state:
    // - Create a channel (use std::sync::mpsc)
    // - Have one "owner" thread that holds the state
    // - Other threads send messages to request changes
    //
    // This is Erlang/Go's approach: constrain SPACE sharing, use TIME (messages)
    //
    // Questions to ponder:
    // - How would Clojure handle shared mutable state? (STM, atoms)
    // - How would Erlang handle it? (processes, message passing)
    // - What axis does each constrain?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
