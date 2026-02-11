//! Exercise 10: Design Your Own Abstraction
//!
//! Create something that requires understanding the full triangle

fn example() {
    println!("Design ideas:");
    println!();
    println!("1. Concurrent hash map");
    println!("   - How to handle resize?");
    println!("   - Lock per bucket or global lock?");
    println!();
    println!("2. Work-stealing queue");
    println!("   - Owner pops from one end");
    println!("   - Stealers pop from other end");
    println!();
    println!("3. Read-copy-update (RCU)");
    println!("   - Readers never block");
    println!("   - Writers copy, modify, swap");
    println!();
    println!("4. Event bus with typed channels");
    println!("   - Subscribe to events by type");
    println!("   - Publishers broadcast to subscribers");
}

fn exercise() {
    // TODO: Design and implement your own abstraction
    //
    // For your design, answer:
    // 1. How is SPACE managed? (allocation, deallocation)
    // 2. How is IDENTITY controlled? (ownership, borrowing, sharing)
    // 3. How is TIME synchronized? (locks, atomics, channels)
    // 4. What coherence strategy? (compile-time, runtime, hardware)
    // 5. Where is unsafe needed? What invariants do you maintain?
    //
    // Start simple, then add complexity as needed.

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
