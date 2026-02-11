use spelled::explicit;

fn example() {
    let x = 5;
    let z = 10;
    explicit! {
        // Combination: let name(rebindable(r)) = coord_shared(x)
        // Meaning: shared coordinates, rebindable
        //
        // Before:                      After r = coord_shared(z):
        // ┌─────────┐                  ┌─────────┐
        // │    5    │ <--- x           │    5    │ <--- x
        // ├─────────┤                  ├─────────┤
        // │   10    │ <--- z           │   10    │ <--- z
        // ├─────────┤                  ├─────────┤
        // │ ptr --->│ (x's slot) <-- r │ ptr --->│ (z's slot) <-- r (retargeted)
        // └─────────┘                  └─────────┘
        let name(rebindable(r)) = coord_shared(x);
        println!("r points to x: {}", r);
        r = coord_shared(z);  // rebind to different SPACE
        println!("r points to z: {}", r);
    }
}

fn exercise() {
    let x = 5;
    let z = 10;
    // Write the real Rust equivalent:
    // let ??? r = ??? x;
    // println!("r points to x: {}", r);
    // r = ??? z;
    // println!("r points to z: {}", r);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
