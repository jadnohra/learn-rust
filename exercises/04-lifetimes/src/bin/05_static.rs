//! Exercise 5: 'static: IDENTITY Valid Forever
//!
//! 'static means: IDENTITY valid for entire program TIME

fn example() {
    let s: &'static str = "hello";  // String literal: lives in binary
    println!("Static string: {}", s);

    // 'static doesn't mean "statically allocated"
    // It means "IDENTITY valid for all program TIME"

    // This works because it's leaked (lives forever):
    let leaked: &'static i32 = Box::leak(Box::new(42));
    println!("Leaked value: {}", leaked);
}

fn exercise() {
    // TODO: Try to create &'static to a local variable
    //
    // let x = 5;
    // let r: &'static i32 = &x;  // What error?
    //
    // Local variables don't live for the entire program.
    // 'static requires the SPACE to exist forever.
    //
    // Question: When would you use Box::leak?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
