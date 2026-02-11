use spelled::explicit;

fn example() {
    let x = 5;
    let y = 10;
    let t = (&x, &y);
    explicit! {
        // Combination: let (owner(a), owner(b)) = (at(t.0), at(t.1))
        // Meaning: get value at coordinates for both, a and b own the copies
        let (owner(a), owner(b)) = (at(t.0), at(t.1));
        println!("a = {}, b = {}", a, b);
    }
}

fn exercise() {
    let x = 5;
    let y = 10;
    let t = (&x, &y);
    // Write the real Rust equivalent:
    // Hint: in real Rust you can use pattern matching: let (&a, &b) = t;
    // let (a, b) = (*t.0, *t.1);
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
