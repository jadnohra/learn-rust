use spelled::explicit;

fn example() {
    let t = (1, 2);
    let r = &t;
    explicit! {
        // Combination: let (owner(a), owner(b)) = at(r)
        // Meaning: get value at coordinates, then unpack, a and b own the copies
        let (owner(a), owner(b)) = at(r);
        println!("a = {}, b = {}", a, b);
    }
}

fn exercise() {
    let t = (1, 2);
    let r = &t;
    // Write the real Rust equivalent:
    // Hint: in real Rust you can use pattern matching: let &(a, b) = r;
    // let (a, b) = *r;
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
