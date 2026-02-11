//! Exercise 1: const vs static: SPACE Existence
//!
//! const: no SPACE exists. Value inlined at each use.
//! static: one SPACE, one address, lives for program TIME.

const CONST_VAL: i32 = 100;
static STATIC_VAL: i32 = 100;

fn example() {
    // Take address of CONST_VAL twice
    let addr1 = &CONST_VAL as *const i32;
    let addr2 = &CONST_VAL as *const i32;
    println!("CONST_VAL addr1: {:p}", addr1);
    println!("CONST_VAL addr2: {:p}", addr2);

    // Take address of STATIC_VAL twice
    let addr3 = &STATIC_VAL as *const i32;
    let addr4 = &STATIC_VAL as *const i32;
    println!("STATIC_VAL addr3: {:p}", addr3);
    println!("STATIC_VAL addr4: {:p}", addr4);

    // Observe:
    // const: addresses might differ (inlined copies)
    // static: addresses always same (one fixed SPACE)
}

fn exercise() {
    // TODO: Create your own const and static values
    // TODO: Take their addresses multiple times
    // TODO: Verify the pattern: const may differ, static always same
    //
    // Questions:
    // - Why does const have no fixed SPACE?
    // - When would you choose static over const?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
