//! Exercise 2: Stack vs Heap: SPACE Location
//!
//! Stack: SPACE tied to scope TIME. LIFO.
//! Heap: SPACE with independent TIME. Flexible.

fn example() {
    let stack_val = 5;           // SPACE on stack
    let heap_val = Box::new(5);  // SPACE on heap, IDENTITY on stack

    println!("stack_val address: {:p}", &stack_val);
    println!("heap_val (Box) address: {:p}", &heap_val);
    println!("*heap_val (contents) address: {:p}", &*heap_val);

    // Observe the address ranges:
    // - stack_val and &heap_val are close (both on stack)
    // - *heap_val is far away (on heap)

    // Which can outlive the function?
    // The heap value can be moved out; the stack value cannot.
}

fn exercise() {
    // TODO: Create several stack values and heap values
    // TODO: Print their addresses and observe the pattern
    //
    // Try:
    // - Multiple stack variables (should be close together)
    // - Multiple Box allocations (heap addresses far from stack)
    // - A Vec and its buffer address (vec.as_ptr())
    //
    // Map to: Stack SPACE ends with scope, Heap SPACE is independent

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
