//! Exercise 9: Fighting the Borrow Checker: Restructure
//!
//! Often the fix is restructuring, not interior mutability

struct Data {
    a: i32,
    b: i32,
}

fn example() {
    let mut data = Data { a: 1, b: 2 };

    // Borrow checker understands struct field disjointness
    let a = &mut data.a;
    let b = &mut data.b;  // This works! Different fields = different SPACE

    *a = 10;
    *b = 20;

    println!("a = {}, b = {}", data.a, data.b);

    // Map to: direct field access lets borrow checker see disjointness
}

fn exercise() {
    // TODO: Try accessing through methods instead
    //
    // impl Data {
    //     fn get_a(&mut self) -> &mut i32 { &mut self.a }
    //     fn get_b(&mut self) -> &mut i32 { &mut self.b }
    // }
    //
    // let a = data.get_a();
    // let b = data.get_b();  // What error?
    //
    // Why does direct field access work but methods don't?
    // Map to: method calls hide the disjointness from the compiler

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
