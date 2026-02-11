//! Exercise 10: When to Use Unsafe
//!
//! Sometimes you know more than the compiler

fn example() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Safe: split_at_mut uses unsafe internally
    let (left, right) = v.split_at_mut(2);
    left[0] = 10;
    right[0] = 20;

    println!("v = {:?}", v);

    // What split_at_mut does internally (simplified):
    // unsafe {
    //     let ptr = v.as_mut_ptr();
    //     let left = slice::from_raw_parts_mut(ptr, mid);
    //     let right = slice::from_raw_parts_mut(ptr.add(mid), len - mid);
    //     (left, right)
    // }

    // The invariant: left and right don't overlap
    // Compiler can't prove it, human can
}

fn exercise() {
    // TODO: Think about when unsafe is appropriate
    //
    // Unsafe is appropriate when:
    // 1. You can prove an invariant the compiler can't
    // 2. You encapsulate it in a safe API
    // 3. The proof is local and verifiable
    //
    // Unsafe is NOT appropriate when:
    // 1. You just want to "make it compile"
    // 2. The invariant depends on external input
    // 3. There's a safe alternative
    //
    // Question: What invariant does split_at_mut rely on?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
