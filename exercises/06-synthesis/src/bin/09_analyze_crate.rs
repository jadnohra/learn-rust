//! Exercise 9: Analyze a Real Crate
//!
//! Pick a crate and analyze its design

fn example() {
    println!("Suggested crates to analyze:");
    println!();
    println!("1. parking_lot - Fast synchronization primitives");
    println!("   Questions:");
    println!("   - How does it improve on std::sync::Mutex?");
    println!("   - What unsafe does it use and why?");
    println!();
    println!("2. crossbeam - Concurrent data structures");
    println!("   Questions:");
    println!("   - How does crossbeam-epoch handle memory reclamation?");
    println!("   - How does crossbeam-channel differ from std::sync::mpsc?");
    println!();
    println!("3. rayon - Data parallelism library");
    println!("   Questions:");
    println!("   - How does it parallelize iterators safely?");
    println!("   - What constraints does it put on closures?");
}

fn exercise() {
    // TODO: Choose a crate and analyze it
    //
    // For your chosen crate, answer:
    // 1. What SPACE x TIME x IDENTITY problems does it solve?
    // 2. What coherence strategy does it use?
    // 3. Where does it use unsafe? Why is it sound?
    // 4. What invariants does the programmer maintain?
    //
    // Write your analysis as comments here, or in a separate document.

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
