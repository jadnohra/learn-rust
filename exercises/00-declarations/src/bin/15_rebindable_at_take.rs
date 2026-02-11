use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let b = Box::new(String::from("hello"));
    explicit! {
        // Combination: let owner(rebindable(x)) = take(at(b))
        // Meaning: get value at coordinates (Box), transfer ownership, rebindable
        //
        // Before (Box<String>):
        //   Stack                 Heap
        // ┌───────┐         ┌─────────────┐         ┌─────────┐
        // │ ptr ──│────────>│ ptr/len/cap │────────>│ "hello" │
        // └───────┘ <-- b   └─────────────┘         └─────────┘
        //   (Box owner)       (String)                (chars)
        //
        // After take(at(b)) - String moved to stack, Box freed:
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "hello" │
        // └─────────────┘ <-- x (owner, rebindable) └─────────┘
        //                 <... b (invalid)
        //
        // During x = String::from("world") - new String created first:
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "hello" │ <-- x
        // ├─────────────┤                      ├─────────┤
        // │ ptr/len/cap │--------------------->│ "world" │ (unnamed)
        // └─────────────┘                      └─────────┘
        //
        // After assignment completes:
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "world" │
        // └─────────────┘ <-- x (owner)        └─────────┘
        // ("hello" dropped, memory freed)
        // Note: compiler may optimize, but this is the semantic model Rust guarantees
        let owner(rebindable(x)) = take(at(b));
        // String::from() creates new owned SPACE on heap
        x = take_or_mem_copy(String::from("world"));
        println!("x = {}", x);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let b = Box::new(String::from("hello"));
    // Write the real Rust equivalent:
    // let ??? x = *b;
    // x = String::from("world");
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
