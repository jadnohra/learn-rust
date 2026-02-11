use spelled::explicit;

fn example() {
    let t = (1, 2);
    explicit! {
        // Combination: let (owner(a), owner(b)) = t
        // Meaning: unpack structure, a and b own the unpacked values
        let (owner(a), owner(b)) = t;
        println!("a = {}, b = {}", a, b);
    }
}

fn exercise() {
    let t = (1, 2);
    // Write the real Rust equivalent:
    // let (a, b) = t;
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
