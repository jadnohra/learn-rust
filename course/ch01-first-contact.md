---
layout: course
title: First Contact
short_title: First Contact
chapter: 1
permalink: /learn-rust/first-contact/
---

# First Contact

This chapter shows what looks the same and what looks different. Most Rust syntax matches C++. Some code that compiles in C++ does not compile in Rust. The compiler errors may seem opaque at first. The reason these programs are rejected, and the mechanism by which the compiler detects the problems, will become clear in later chapters.

If you've written C++, most Rust syntax will feel familiar. Functions, structs, loops, and vectors work the same way.

```rust
fn main() {
    println!("Hello, world!");
}
```

```rust
fn main() {
    let x = 5;
    let y = 10;
    println!("{} + {} = {}", x, y, x + y);
}
```

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(3, 4);
    println!("Result: {}", result);
}
```

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", v);
    println!("Length: {}", v.len());
}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point at ({}, {})", p.x, p.y);
}
```

```rust
fn main() {
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
}
```

<details>
<summary>Checkpoint</summary>
<p>Sees Rust syntax. Recognizes it. Can read it.</p>
</details>

---

## Examples That Do Not Compile

These examples do not compile:

```rust
fn main() {
    let v = vec![1, 2, 3];
    let w = v;
    println!("{:?}", v);
}
```

```
error[E0382]: borrow of moved value: `v`
```

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let r = &v[0];
    v.push(4);
    println!("{}", r);
}
```

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r);
}
```

```
error[E0597]: `x` does not live long enough
```

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

```rust
fn get_string() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let s = get_string();
}
```

```
error[E0106]: missing lifetime specifier
```

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        v.push(*x);
    }
}
```

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let closure = || v.push(4);
    println!("{:?}", v);
    closure();
}
```

```
error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable
```

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    thread::spawn(|| {
        println!("{:?}", v);
    });
}
```

```
error[E0373]: closure may outlive the current function
```

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let rc = Rc::new(5);
    thread::spawn(move || {
        println!("{}", rc);
    });
}
```

```
error[E0277]: `Rc<i32>` cannot be sent between threads safely
```

None of these compile in Rust. They compile in C++ and contain memory bugs.

<details>
<summary>Checkpoint</summary>
<p>Sees buggy code. Knows the bugs. Sees Rust rejects it. Does not know how Rust detects them.</p>
</details>

---

## The Dangling Reference Case

The dangling reference case:

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r);
}
```

`r` refers to `x`. The block ends. `x` is gone. `r` points to nothing.

The C++ equivalent compiles and runs. It prints garbage, or crashes, or prints 5 by accident. The behavior is undefined.

```cpp
int main() {
    int* r;
    {
        int x = 5;
        r = &x;
    }
    printf("%d\n", *r);
}
```

<details>
<summary>Checkpoint</summary>
<p>Sees one example explained. Connects Rust error to C++ undefined behavior. Knows Rust catches bugs C++ misses. Does not know how yet. Curious.</p>
</details>
