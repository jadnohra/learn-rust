use spelled::explicit;

fn example() {
    let x = 5;
    explicit! {
        let name(r) = coord_shared(x);
        // Combination: let owner(y) = mem_copy(at(r))
        // Meaning: get value at coordinates, then duplicate, y owns the copy
        let owner(y) = mem_copy(at(r));
        println!("x = {}, y = {}", x, y);
    }
}

fn exercise() {
    let x = 5;
    // Write the real Rust equivalent:
    // let r = ??? x;
    // let y = *r;
    // println!("x = {}, y = {}", x, y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
