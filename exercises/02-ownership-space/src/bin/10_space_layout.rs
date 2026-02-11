//! Exercise 10: SPACE Layout: Where Are Things?
//!
//! Observe how SPACE is laid out

fn example() {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: Box<i32> = Box::new(3);

    println!("a at {:p}", &a);
    println!("b at {:p}", &b);
    println!("c (the Box itself) at {:p}", &c);
    println!("*c (heap contents) at {:p}", &*c);

    // Observe: a, b, &c are close together (stack)
    // Observe: *c is far away (heap)

    let d = vec![1, 2, 3, 4, 5];
    println!("\nVec d at {:p}", &d);
    println!("d's buffer at {:p}", d.as_ptr());
    println!("d[0] at {:p}", &d[0]);

    // Vec is similar: the Vec struct is on stack, buffer is on heap
}

fn exercise() {
    // TODO: Create a nested structure and visualize its layout
    //
    // Try something like:
    // struct Inner { value: i32 }
    // struct Outer { inner: Inner, boxed: Box<Inner> }
    //
    // Print addresses of:
    // - The Outer struct
    // - The inner field
    // - The boxed field (the Box itself)
    // - The boxed contents (*boxed)
    //
    // Draw a diagram of where each piece lives (stack vs heap)

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
