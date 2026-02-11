---
layout: course
title: Exercises
nav_title: Exercises
permalink: /learn-rust/exercises/
wide: true
---

# Exercises

> **This page and its exercises are a work in progress.**

## How these are organized

Most Rust courses organize exercises around language features. Structs, then enums, then traits, then generics, then lifetimes, then concurrency. The progression follows the language's syntax, from simple constructs to complex ones.

These exercises are organized around the framework. The progression follows space, time, coordinates, and coherence, and introduces language features as they become necessary to express each idea. You will encounter threads, mutexes, and memory ordering long before you would in a conventional Rust course, because the framework demands them early.

Declarations come first because they are the vocabulary. Foundations come second because they connect the vocabulary to the problems Rust exists to solve. Ownership space, interior mutability, lifetimes, and memory ordering each explore one region of the design space in depth. Synthesis ties them together.

## The sections

**[0 · Declarations.](#0--declarations)** The `explicit!` notation and its mapping to real Rust. Every combination of `owner`/`name`, `take`/`mem_copy`/`coord_shared`/`coord_exclusive`, and `rebindable`. 21 exercises.

**[1 · Foundations.](https://github.com/jadnohra/learn-rust/tree/main/exercises/01-foundations/src/bin)** Derived data, the borrow rule, identity and validity, shadowing, move semantics, runtime coherence, threads, mutexes, memory ordering, and language design choices. The problems Rust exists to solve, seen from first principles. 10 exercises.

**[2 · Ownership Space.](https://github.com/jadnohra/learn-rust/tree/main/exercises/02-ownership-space/src/bin)** Where values live. `const` vs `static`, stack vs heap, `Box`, `Rc`, `RefCell`, `Arc`, `Mutex`, `Weak`, drop order, and memory layout. 10 exercises.

**[3 · Interior Mutability.](https://github.com/jadnohra/learn-rust/tree/main/exercises/03-interior-mutability/src/bin)** `Cell`, `RefCell`, `Mutex`, `RwLock`, and the spectrum between compile-time and runtime enforcement. Shared mutation still obeys the borrow rule, but the enforcement moves to runtime. 10 exercises.

**[4 · Lifetimes.](https://github.com/jadnohra/learn-rust/tree/main/exercises/04-lifetimes/src/bin)** Validity, multiple inputs, elision rules, lifetimes in structs, `'static`, non-lexical lifetimes, value tracking, conservative analysis, restructuring code, and `unsafe` as an escape hatch. 10 exercises.

**[5 · Memory Ordering.](https://github.com/jadnohra/learn-rust/tree/main/exercises/05-memory-ordering/src/bin)** Visibility, store buffers, release/acquire, `SeqCst`, `Relaxed`, spinlocks, double-checked locking, hardware models, acquire-release pairs, and lock-free queues. 10 exercises.

**[6 · Synthesis.](https://github.com/jadnohra/learn-rust/tree/main/exercises/06-synthesis/src/bin)** Coherence strategies, weak reference cycles, thread pools, choosing between type strategies, implementing `Rc` from scratch, message passing vs shared state, the observer pattern, a lock-free stack, analyzing a real crate, and designing your own ownership scheme. 10 exercises.

---

## 0 · Declarations

> **This page is a work in progress.** Rust notation is compact because it is minimal and overloaded. The `spelled` crate undoes that compression for clarity. If you know Rust syntax but it feels tedious and convoluted, this page may help. We are working on expanding and explaining it better.

Rust hides a lot behind simple syntax. `let y = x` might copy the value, move it, or borrow it. The type determines which one, and the syntax gives no indication.

This is a design choice. Experienced Rust programmers carry the distinctions in their head. A learner does not have those distinctions yet, and learning them from syntax that hides them is working uphill.

The course includes a companion crate called `spelled` — as in spelling out what happens. A macro called `explicit!` expands Rust's implicit syntax into a form where every operation says what it does.

```rust
explicit! {
    let owner(y) = take_or_mem_copy(x);
}
```

This compiles and runs. It does the same thing as `let y = x`, but it says what is happening.

### The left side

The left side of a `let` says what the binding receives.

- `owner(y)` — y receives ownership. It controls when the space dies.
- `name(r)` — r receives coordinates to space owned elsewhere. A reference.
- `rebindable()` — the binding can be pointed at something else later.

### The right side

The right side says what happens to the source.

- `take(x)` — transfers ownership. x becomes invalid.
- `mem_copy(x)` — duplicates the bytes. x stays valid.
- `take_or_mem_copy(x)` — lets the compiler decide based on the type. Copy types get copied, everything else gets moved.
- `coord_shared(x)` — creates shared coordinates. Multiple allowed at the same time.
- `coord_exclusive(x)` — creates exclusive coordinates. Only one allowed.
- `at(r)` — follows coordinates to get the value.

### The exercises

Each exercise shows the explicit version first, then asks you to write the real Rust equivalent. You learn by seeing what the translation erases.

<details markdown="1">
<summary>01 · literal</summary>

```rust
use spelled::explicit;

fn example() {
    explicit! {
        // Combination: let owner(x) = 5
        // Meaning: new SPACE with literal value, x owns it
        //
        // Stack:
        // ┌─────────┐
        // │    5    │ <--- x (owner)
        // └─────────┘
        let owner(x) = 5;
        println!("x = {}", x);
    }
}

fn exercise() {
    // Write the real Rust equivalent:
    // let ??? x = 5;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>02 · rebindable literal</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    explicit! {
        // Combination: let owner(rebindable(x)) = 5
        // Meaning: new SPACE, x owns it, rebindable
        //
        // Before:            After x = 10:
        // ┌─────────┐        ┌─────────┐
        // │    5    │ <-- x  │   10    │ <-- x  (same slot, new value)
        // └─────────┘        └─────────┘
        let owner(rebindable(x)) = 5;
        x = take_or_mem_copy(10);
        println!("x = {}", x);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    // Write the real Rust equivalent:
    // let ??? x = 5;
    // x = 10;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>03 · mem copy</summary>

```rust
use spelled::explicit;

fn example() {
    let x = 5;
    explicit! {
        // Combination: let owner(y) = mem_copy(x)
        // Meaning: duplicate SPACE, x remains valid, y owns the copy
        //
        // Stack:
        // ┌─────────┐
        // │    5    │ <--- x (owner)
        // ├─────────┤
        // │    5    │ <--- y (owner of copy)
        // └─────────┘
        //
        // Both bindings own independent SPACE
        let owner(y) = mem_copy(x);
        println!("x = {}, y = {}", x, y);
    }
}

fn exercise() {
    let x = 5;
    // Write the real Rust equivalent of: let owner(y) = mem_copy(x)
    // let y = ??? x;
    // println!("x = {}, y = {}", x, y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>04 · rebindable mem copy</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let x = 5;
    explicit! {
        // Combination: let owner(rebindable(y)) = mem_copy(x)
        // Meaning: duplicate SPACE, y owns it, rebindable
        let owner(rebindable(y)) = mem_copy(x);
        y = take_or_mem_copy(20);
        println!("x = {}, y = {}", x, y);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let x = 5;
    // Write the real Rust equivalent:
    // let ??? y = ??? x;
    // y = 20;
    // println!("x = {}, y = {}", x, y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>05 · take</summary>

```rust
use spelled::explicit;

fn example() {
    let x = String::from("hello");
    explicit! {
        // Combination: let owner(y) = take(x)
        // Meaning: transfer ownership, x invalidated, y now owns
        //
        // Before take(x):
        //      Stack                     Heap
        // ┌───────────────┐        ┌─────────────┐
        // │ ptr ----------│------->│ "hello"     │
        // │ len: 5        │ <-- x  └─────────────┘
        // │ cap: 5        │    (owner)
        // └───────────────┘
        //
        // After take(x):
        //      Stack                     Heap
        // ┌───────────────┐        ┌─────────────┐
        // │ ptr ----------│------->│ "hello"     │  (same heap)
        // │ len: 5        │ <-- y  └─────────────┘
        // │ cap: 5        │    (owner) <... x (invalid)
        // └───────────────┘
        let owner(y) = take(x);
        println!("y = {}", y);
        // println!("x = {}", x);  // won't compile: x moved
    }
}

fn exercise() {
    let x = String::from("hello");
    // Write the real Rust equivalent:
    // let y = ??? x;
    // println!("y = {}", y);
    // // println!("x = {}", x);  // won't compile: x moved
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>06 · rebindable take</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let x = String::from("hello");
    explicit! {
        // Combination: let owner(rebindable(y)) = take(x)
        // Meaning: transfer ownership to y, rebindable
        let owner(rebindable(y)) = take(x);
        // String::from() creates new owned SPACE on heap
        y = take_or_mem_copy(String::from("world"));
        println!("y = {}", y);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let x = String::from("hello");
    // Write the real Rust equivalent:
    // let ??? y = ??? x;
    // y = String::from("world");
    // println!("y = {}", y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>07 · take or mem copy</summary>

```rust
use spelled::explicit;

fn example() {
    let x1 = 5;                        // i32: Copy
    let x2 = String::from("hello");    // String: Move
    explicit! {
        // Combination: let owner(y) = take_or_mem_copy(x)
        // Meaning: compiler decides based on type, y owns result
        //
        // i32 (Copy):               String (Move):
        // ┌─────────┐               ┌───────────┐        ┌─────────┐
        // │    5    │ <-- x1        │ ptr ------│------->│ "hello" │
        // ├─────────┤    (owner)    │ len: 5    │ <-- y2 └─────────┘
        // │    5    │ <-- y1        │ cap: 5    │    (owner) <... x2 (invalid)
        // └─────────┘    (owner)    └───────────┘
        // x1 still valid            (same heap, moved)
        let owner(y1) = take_or_mem_copy(x1);
        let owner(y2) = take_or_mem_copy(x2);

        println!("x1 = {}, y1 = {}", x1, y1);  // x1 still valid (was copied)
        println!("y2 = {}", y2);
        // println!("x2 = {}", x2);  // won't compile: x2 moved
    }
}

fn exercise() {
    let x1 = 5;                        // i32: Copy
    let x2 = String::from("hello");    // String: Move
    // Write the real Rust equivalent:
    // Hint: real Rust uses the same syntax for both—the type determines behavior
    // let y1 = x1;
    // let y2 = x2;
    // println!("x1 = {}, y1 = {}", x1, y1);
    // println!("y2 = {}", y2);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>08 · coord shared</summary>

```rust
use spelled::explicit;

fn example() {
    let x = 5;
    explicit! {
        // Combination: let name(r) = coord_shared(x)
        // Meaning: create shared coordinates to x's SPACE, fixed label
        //
        // Stack:
        // ┌─────────┐
        // │    5    │ <--- x
        // ├─────────┤
        // │ ptr  ---│---> (x's slot)  <--- r1 (shared)
        // ├─────────┤
        // │ ptr  ---│---> (x's slot)  <--- r2 (shared)
        // └─────────┘
        //
        // Multiple shared coordinates to same SPACE: OK
        let name(r1) = coord_shared(x);
        let name(r2) = coord_shared(x);  // OK: multiple shared coordinates allowed
        println!("r1 = {}, r2 = {}, x = {}", r1, r2, x);
    }
}

fn exercise() {
    let x = 5;
    // Write the real Rust equivalent:
    // let r1 = ??? x;
    // let r2 = ??? x;
    // println!("r1 = {}, r2 = {}, x = {}", r1, r2, x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>09 · rebindable coord shared</summary>

```rust
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
```

</details>

<details markdown="1">
<summary>10 · coord exclusive</summary>

```rust
use spelled::explicit;

fn example() {
    let mut y = 5;
    explicit! {
        // Combination: let name(r) = coord_exclusive(y)
        // Meaning: create exclusive coordinates to y's SPACE, mutation allowed, fixed label
        //
        // 1. Before borrow:
        // ┌─────────┐
        // │    5    │ <--- y
        // └─────────┘
        //
        // 2. During borrow (y suspended):
        // ┌─────────┐
        // │    5    │ <... y (suspended while r lives)
        // ├─────────┤
        // │ ptr --->│ (y's slot) <--- r (exclusive)
        // └─────────┘
        //
        // 3. After r drops (y restored):
        // ┌─────────┐
        // │   10    │ <--- y (usable again)
        // └─────────┘
        //
        // Key: borrow = temporary suspension, not permanent deletion like move
        let name(r) = coord_exclusive(y);
        at(r) = 10;  // mutate through exclusive coordinates
        println!("y = {}", y);
    }
}

fn exercise() {
    let mut y = 5;
    // Write the real Rust equivalent:
    // let r = ??? y;
    // *r = 10;
    // println!("y = {}", y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>11 · rebindable coord exclusive</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let mut a = 5;
    let mut b = 10;
    explicit! {
        // Combination: let name(rebindable(r)) = coord_exclusive(y)
        // Meaning: exclusive coordinates, rebindable
        //
        // Before:                       After at(r)=100, r=coord_exclusive(b):
        // ┌─────────┐                   ┌─────────┐
        // │    5    │ <--- a            │   100   │ <--- a (mutated via r)
        // ├─────────┤                   ├─────────┤
        // │   10    │ <--- b            │   10    │ <--- b
        // ├─────────┤                   ├─────────┤
        // │ ptr --->│ (a) <-- r (excl)  │ ptr --->│ (b) <-- r (retargeted)
        // └─────────┘                   └─────────┘
        let name(rebindable(r)) = coord_exclusive(a);
        at(r) = 100;
        r = coord_exclusive(b);  // rebind to different SPACE
        at(r) = 200;
        println!("a = {}, b = {}", a, b);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let mut a = 5;
    let mut b = 10;
    // Write the real Rust equivalent:
    // let ??? r = ??? a;
    // *r = 100;
    // r = ??? b;
    // *r = 200;
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>12 · at mem copy</summary>

```rust
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
```

</details>

<details markdown="1">
<summary>13 · rebindable at mem copy</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let x = 5;
    explicit! {
        let name(r) = coord_shared(x);
        // Combination: let owner(rebindable(y)) = mem_copy(at(r))
        // Meaning: get value at coordinates, duplicate, y owns it, rebindable
        let owner(rebindable(y)) = mem_copy(at(r));
        y = take_or_mem_copy(100);
        println!("x = {}, y = {}", x, y);  // x unchanged, y modified
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let x = 5;
    // Write the real Rust equivalent:
    // let r = ??? x;
    // let ??? y = *r;
    // y = 100;
    // println!("x = {}, y = {}", x, y);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>14 · at take</summary>

```rust
use spelled::explicit;

fn example() {
    let b = Box::new(String::from("hello"));
    explicit! {
        // Combination: let owner(x) = take(at(b))
        // Meaning: get value at coordinates (Box), transfer ownership to x
        //
        // Before take(at(b)):
        //   Stack                  Heap                    Heap
        // ┌─────────┐        ┌─────────────┐         ┌─────────┐
        // │ ptr ----│------->│ ptr/len/cap │-------->│ "hello" │
        // └─────────┘ <-- b  └─────────────┘         └─────────┘
        //   (Box owner)        (String)
        //
        // After take(at(b)):
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "hello" │
        // └─────────────┘ <-- x (owner)        └─────────┘
        //                 <... b (invalid)
        // x owns String directly (same heap data)
        let owner(x) = take(at(b));
        println!("x = {}", x);
        // println!("b = {:?}", b);  // won't compile: b moved
    }
}

fn exercise() {
    let b = Box::new(String::from("hello"));
    // Write the real Rust equivalent:
    // Hint: You can move out of a Box by dereferencing it
    // let x = *b;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>15 · rebindable at take</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let b = Box::new(String::from("hello"));
    explicit! {
        // Combination: let owner(rebindable(x)) = take(at(b))
        // Meaning: get value at coordinates (Box), transfer ownership, rebindable
        //
        // Before (Box<String>):
        //   Stack                 Heap
        // ┌───────┐         ┌─────────────┐         ┌─────────┐
        // │ ptr ──│────────>│ ptr/len/cap │────────>│ "hello" │
        // └───────┘ <-- b   └─────────────┘         └─────────┘
        //   (Box owner)       (String)                (chars)
        //
        // After take(at(b)) - String moved to stack, Box freed:
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "hello" │
        // └─────────────┘ <-- x (owner, rebindable) └─────────┘
        //                 <... b (invalid)
        //
        // During x = String::from("world") - new String created first:
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "hello" │ <-- x
        // ├─────────────┤                      ├─────────┤
        // │ ptr/len/cap │--------------------->│ "world" │ (unnamed)
        // └─────────────┘                      └─────────┘
        //
        // After assignment completes:
        //   Stack                                    Heap
        // ┌─────────────┐                      ┌─────────┐
        // │ ptr/len/cap │--------------------->│ "world" │
        // └─────────────┘ <-- x (owner)        └─────────┘
        // ("hello" dropped, memory freed)
        // Note: compiler may optimize, but this is the semantic model Rust guarantees
        let owner(rebindable(x)) = take(at(b));
        // String::from() creates new owned SPACE on heap
        x = take_or_mem_copy(String::from("world"));
        println!("x = {}", x);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let b = Box::new(String::from("hello"));
    // Write the real Rust equivalent:
    // let ??? x = *b;
    // x = String::from("world");
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>16 · tuple unpack</summary>

```rust
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
```

</details>

<details markdown="1">
<summary>17 · tuple at second</summary>

```rust
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
```

</details>

<details markdown="1">
<summary>18 · tuple at both</summary>

```rust
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
```

</details>

<details markdown="1">
<summary>19 · at then unpack</summary>

```rust
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
```

</details>

<details markdown="1">
<summary>20 · tuple rebindable</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let t = (1, 2);
    explicit! {
        // Combination: let (owner(rebindable(a)), owner(rebindable(b))) = t
        // Meaning: unpack, both own their values, both rebindable
        let (owner(rebindable(a)), owner(rebindable(b))) = t;
        a = take_or_mem_copy(100);
        b = take_or_mem_copy(200);
        println!("a = {}, b = {}", a, b);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let t = (1, 2);
    // Write the real Rust equivalent:
    // let (??? a, ??? b) = t;
    // a = 100;
    // b = 200;
    // println!("a = {}, b = {}", a, b);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>

<details markdown="1">
<summary>21 · litmus test</summary>

```rust
use spelled::explicit;

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn example() {
    let mut y = 5;
    let r = &mut y;
    explicit! {
        // The litmus test: Rust's most confusing declaration syntax
        //
        // Current Rust: let mut &mut x = r;
        //   - mut after let: binding can change
        //   - &mut in pattern: match a mutable reference, extract target
        //
        // What it actually does:
        //   - r must be &mut T
        //   - get value at coordinates
        //   - bind x to a copy of the target (x owns the copy)
        //   - x can be rebound later
        let owner(rebindable(x)) = mem_copy(at(r));
        x = take_or_mem_copy(100);
        println!("x = {}", x);
    }
}

// Allow unused_assignments: This exercise intentionally demonstrates reassignment,
// so the initial value is overwritten before being read.
#[allow(unused_assignments)]
fn exercise() {
    let mut y = 5;
    let r = &mut y;
    // Write the real Rust equivalent:
    // let ??? x = *r;
    // x = 100;
    // println!("x = {}", x);
    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
```

</details>
