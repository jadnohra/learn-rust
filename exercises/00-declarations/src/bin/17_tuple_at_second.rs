use spelled::explicit;

fn example() {
    let x = 5;
    let t = (1, &x);
    explicit! {
        // Combination: let (owner(a), owner(b)) = (t.0, at(t.1))
        // Meaning: pick first, get value at coordinates for second, both own their values
        let (owner(a), owner(b)) = (t.0, at(t.1));
        println!("a = {}, b = {}", a, b);
    }
}

fn exercise() {
    let x = 5;
    let t = (1, &x);
    // Write the real Rust equivalent:
    // Hint: in real Rust you can use pattern matching: let (a, &b) = t;
    // let (a, b) = (t.0, *t.1);
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
