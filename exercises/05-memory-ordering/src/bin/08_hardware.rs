//! Exercise 8: The Hardware Reality
//!
//! Memory ordering exists because hardware defers coherence

fn example() {
    println!("Memory ordering abstracts over hardware differences:");
    println!();
    println!("Store buffers: write to local queue, drain later");
    println!("Caches: each core has copy, must sync via MESI");
    println!("Reordering: CPU executes out of order for performance");
    println!();
    println!("x86: relatively strong ordering (TSO)");
    println!("  - Stores are not reordered with other stores");
    println!("  - Loads are not reordered with other loads");
    println!("  - Most orderings are 'free'");
    println!();
    println!("ARM/RISC-V: weaker ordering");
    println!("  - More reordering allowed");
    println!("  - Barriers inserted for Acquire/Release/SeqCst");
    println!();
    println!("Rust's memory model abstracts over hardware:");
    println!("Ordering::SeqCst = 'whatever barriers this platform needs'");
}

fn exercise() {
    // TODO: Research and answer:
    //
    // 1. What is a store buffer and why does it exist?
    // 2. What is the MESI protocol?
    // 3. Why is ARM weaker than x86?
    // 4. What instruction does x86 use for SeqCst stores?
    //
    // Bonus: Compile a simple atomic operation for x86 and ARM
    // cargo rustc --release --target x86_64-unknown-linux-gnu -- --emit asm
    // cargo rustc --release --target aarch64-unknown-linux-gnu -- --emit asm

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
