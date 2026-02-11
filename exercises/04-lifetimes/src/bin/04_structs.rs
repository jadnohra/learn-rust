//! Exercise 4: Structs with References: IDENTITY in a Container
//!
//! A struct holding a reference: the struct's TIME is bounded by the reference's

struct Holder<'a> {
    value: &'a i32,
}

fn example() {
    let x = 5;
    let holder = Holder { value: &x };
    println!("holder.value = {}", holder.value);

    // The struct can't outlive what it references
    // Map to: struct's TIME <= contained IDENTITY's TIME
}

fn exercise() {
    // TODO: Create a situation where holder outlives the referenced data
    //
    // let holder;
    // {
    //     let y = 10;
    //     holder = Holder { value: &y };
    // }
    // println!("{}", holder.value);  // What error?
    //
    // Map to: struct containing IDENTITY must not outlive the SPACE

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
