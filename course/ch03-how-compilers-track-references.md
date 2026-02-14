---
layout: course
title: Catching Coherence at Compile Time
short_title: Compiler Analysis
chapter: 3
permalink: /learn-rust/how-compilers-track-references/
---

# Catching Coherence at Compile Time

<div class="toc" markdown="0">
<p class="toc-title">Contents</p>
<ul>
<li><a href="#bindings-and-addresses">Bindings and Addresses</a></li>
<li><a href="#why-you-need-coordinates">Why You Need Coordinates</a></li>
<li><a href="#detecting-dead-space">Detecting Dead Space</a></li>
<li><a href="#memory-architecture-and-compiler-analysis">Memory Architecture and Compiler Analysis</a></li>
<li><a href="#why-c-compilers-skip-this-analysis">Why C++ Compilers Skip This Analysis</a></li>
<li><a href="#how-other-languages-approach-this">How Other Languages Approach This</a></li>
<li><a href="#how-rust-enables-the-analysis">How Rust Enables the Analysis</a></li>
<li><a href="#using-data-managing-space">Using Data, Managing Space</a></li>
</ul>
</div>

Chapter 1 showed bugs that Rust catches and C++ misses. Chapter 2 explained why these bugs exist. They are coherence failures between SPACE, TIME, and COORDINATES. Most involve coordinates pointing to space that no longer exists.

This chapter examines how a compiler can catch these bugs, what it needs to track, why C++ compilers skip this analysis, and how Rust makes it possible.

<details>
<summary>Checkpoint</summary>
<p>You understand this chapter explains how a compiler could catch coordinate coherence failures. You are motivated by the question: what would that require?</p>
</details>

---

## Bindings and Addresses

Chapter 1 ended with a dangling reference. We analyze it now.

```cpp
int* r;
{
    int x = 5;
    r = &x;
}
std::cout << *r;
```

The program has two bindings. The binding `x` holds a value. The binding `r` holds an address, the location of `x`. When `x` goes out of scope, its memory is reclaimed. The binding `r` still exists and still holds an address, but that address no longer points to valid memory.

Both bindings are instantiations: a name bound to a location holding something. They differ in what that something is.

| Name | Location | Holds |
|------|----------|-------|
| `x` | A | `5` (a value) |
| `r` | B | address of A (this is a COORDINATE) |

Through `r`, you can access `x`. Each binding has its own name, location, and lifetime. Taking an address creates this situation: a new binding that refers to space controlled by another binding. The coordinate has its own lifetime. The space it points to has a different lifetime. When those lifetimes fall out of sync, coherence fails. Coordinates create a coherence obligation between independent lifetimes.

<details>
<summary>Checkpoint</summary>
<p>You see the ch01 dangling reference analyzed in detail. You understand two bindings: <code>x</code> holds a value, <code>r</code> holds a COORDINATE. You understand that taking an address creates a coherence obligation between independent lifetimes.</p>
</details>

---

## Why You Need Coordinates

We use "coordinate" rather than "pointer" or "reference" because languages define those terms differently. In C++, `T*` is a pointer and `T&` is a reference. In Rust, `&T` is a reference and `*const T` is a raw pointer. The conceptual problem is identical: something that tells you where data lives. "Coordinate" abstracts over these language-specific terms.

Coordinates are not optional. Copying large data costs time and energy, so programs pass addresses instead of values. Data structures like graphs, trees, and linked lists require indirection because you cannot inline a cycle. Any language that cares about performance or expressiveness needs coordinates.

Coordinates are syntactically independent and semantically dependent. Syntactically independent means `r` has its own declaration, its own location, its own scope, and the compiler processes it separately from `x`. Semantically dependent means the purpose of `r` is to refer to `x`, and without `x`, the address in `r` points to nothing meaningful. One way that coherence problems arise is semantics that syntax does not capture.

<details>
<summary>Checkpoint</summary>
<p>You understand why we use "coordinate" as a term. You know coordinates are unavoidable: physics makes copying expensive, data structures require indirection. You understand coordinates are syntactically independent and semantically dependent. You see that coherence problems arise from semantics that syntax does not capture.</p>
</details>

---

## Detecting Dead Space

In imperative languages, source text has hierarchical structure. The grammar defines how text parses into functions, blocks, statements, expressions. Names exist within this hierarchy. A variable declared inside a block is visible only within that block and any blocks nested inside it. When the block ends, the name goes out of scope. The lifetime of a variable is bounded by the scope that declares it.

The RAM model has no such structure. Memory is a sequence of numbered cells. Any address can read or write any cell. Nothing in the machine knows that a particular cell was allocated for a variable named `x` in a particular scope. Nothing prevents reading that cell after the scope ends. The structure exists in the language, not in the machine.

The compiler bridges them. It translates the scoped, named world of source text into the flat, addressed world of the RAM model. During this translation, the compiler builds intermediate representations that preserve the source structure. One such representation is the control flow graph.

A control flow graph represents execution paths through the program. Nodes are statements. Edges connect statements where execution can flow from one to the next. Branches create multiple outgoing edges. Loops create edges that point backward.

The compiler can analyze this graph to detect dead space access. The analysis tracks by variable name. When the compiler sees `r = &x`, it records that coordinate `r` points to `x`'s space. When it sees `x`'s scope end, it marks that `x`'s space dies. When it sees `*r`, it knows this accesses `x`'s space. A path in the graph corresponds to a possible execution sequence in the program. If any path leads from the death of `x`'s space to the access through `r`, the program might execute that sequence and dereference a coordinate whose space is gone. The compiler does not know which paths the program will actually take at runtime, so it rejects if any path is unsafe.

This is one class of problem that coordinate analysis can detect. There are others, covered later. We start here because the dangling reference is the clearest case.

We trace through the dangling reference from Chapter 1.

```rust
fn main() {
    let r;
    {
        let x = 5;       // A
        r = &x;          // B
    }                    // C
    println!("{}", r);   // D
}
```

```
       ┌───┐
       │ A │ let x = 5
       └─┬─┘
         ▼
       ┌───┐
       │ B │ r = &x
       └─┬─┘
         ▼
       ┌───┐
       │ C │ ✗ x dies (scope ends)
       └─┬─┘
         ▼
       ┌───┐
       │ D │ *r ← ACCESS with dead space ✗
       └───┘
```

The path from C to D exists. The compiler rejects.

A more complex example with branching:

```rust
fn example(flag: bool) {
    let r;

    if flag {
        let x = 5;           // B
        r = &x;
        println!("{}", *r);  // D: safe access, x alive
    } else {
        let y = 10;          // C
        r = &y;
    }                        // C: y dies

    println!("{}", *r);      // G: unsafe if else branch taken
}
```

```
                   ┌───┐
                   │ A │ if flag
                   └─┬─┘
             ┌──────┴──────┐
             ▼             ▼
           ┌───┐         ┌───┐
           │ B │         │ C │
           │x=5│         │y=10│
           │r=&x│        │r=&y│
           └─┬─┘         │ ✗ │ ← y dies (scope ends)
             │           └─┬─┘
             ▼             ▼
           ┌───┐         ┌───┐
           │ D │         │ F │
           │*r │         │ ✗ │ ← propagated
           │safe│        └─┬─┘
           └─┬─┘           │
             │             │
             └──────┬──────┘
                    ▼
                  ┌───┐
                  │ G │
                  │*r │ ← ACCESS, unsafe on else path ✗
                  └───┘
```

The access at D is safe because no path from a death reaches it. The access at G is unsafe because a path from C reaches it through F. The compiler finds the unsafe path and rejects.

<details>
<summary>Checkpoint</summary>
<p>You understand source text has hierarchical structure that the RAM model lacks. You know the compiler bridges them using intermediate representations like the control flow graph. You understand dead space detection as graph reachability: can any path lead from space death to coordinate access? You see both the simple linear example and the branching example.</p>
</details>

---

## Memory Architecture and Compiler Analysis

The RAM model is flat, but programs organize memory into regions with different properties.

Static space lives for the entire program. Coordinates to static space are always valid.

Stack space dies when scopes end. The compiler sees scope boundaries in the source text.

Heap space dies when freed. Without constraints, the coordinate to heap space can travel through the program, and any code holding it can free it. No lexical structure governs heap. Why compilers cannot simply track this, and how Rust constrains it, are questions we address shortly.

```
STACK ONLY (tree)              HEAP ONLY (graph)           WHAT WE HAVE (hybrid)

    main                        ┌───┐   ┌───┐                 main
    ├── foo                     │ A │◄──│ B │             ├── foo ──────┐
    │   └── bar                 └─┬─┘   └───┘             │   └── bar   │
    │       └── baz               │       ▲               │             ▼
    └── qux                       ▼       │               └── qux    [heap]
                                ┌───┐   ┌─┴─┐                          ▲
                                │ C │──►│ D │                          │
                                └───┘   └───┘              ────────────┘

Coords point UP only.          Coords point ANYWHERE.      Stack: tree. Heap: escapes.
```

For dead space detection, the compiler can determine from the source text when stack coordinates are valid. Heap has no such property. The compiler cannot see when heap space dies.

The stack's lexical structure makes functions natural units for dead space detection.

<details>
<summary>Checkpoint</summary>
<p>You understand memory regions: static (always valid), stack (lexical structure visible), heap (no lexical structure). You see the visual contrast between stack-only, heap-only, and hybrid memory. You know the compiler can see when stack coordinates are valid but not heap. You understand that stack's lexical structure makes functions natural units for dead space detection.</p>
</details>

---

## Why C++ Compilers Skip This Analysis

The algorithm for dead space detection is straightforward. Build the control flow graph, track where coordinates point, mark where space dies, check if any path leads from death to access. Most compilers do not do this.

The way C and C++ compilers operate prevents building a complete control flow graph.

**Separate compilation.** Each source file becomes an object file independently. When the compiler processes a call to a function defined in another file, it sees only the declaration, not the definition. The object file contains a reference to an external symbol, resolved later by the linker. By link time, analysis is over. The control flow graph has edges that lead to functions whose bodies are invisible.

**Function pointers.** The call target is a runtime value. The compiler cannot know which function will execute. The control flow graph has edges to unknown destinations.

Without the complete graph, the analysis cannot trace all paths.

**Unconstrained pointers.** Even with a complete graph, the compiler needs to track which coordinate points to which space. When the compiler sees `r = &x`, it can record that `r` points to `x`'s space. But C++ allows coordinates to be created without naming a target. An integer can be cast to a pointer, and the compiler has no record of what space that address refers to. Pointer arithmetic can compute addresses the compiler cannot trace back to their origin. Type erasure through `void*` discards the type information the compiler uses to track coordinates. The analysis requires knowing where every coordinate points. C++ provides no such guarantee.

```cpp
int* p = (int*)0x1234;           // coordinate from integer
int* q = p + 5;                   // coordinate from arithmetic
void* v = p;                      // type erased
int** stored = &p;                // coordinate to coordinate
```

The next chapter examines why perfect analysis is impossible, even without these obstacles.

<details>
<summary>Checkpoint</summary>
<p>You understand why C++ compilers cannot do this analysis. You know the three obstacles: separate compilation (incomplete graph), function pointers (unknown destinations), unconstrained pointers (coordinates from nowhere). You understand that even with a complete graph, tracking is impossible without constraints on pointer operations.</p>
</details>

---

## How Other Languages Approach This

**No analysis.** C and C++ compilers. The obstacles we described prevent it. Some compilers warn about obvious cases like returning addresses of locals, but systematic tracking does not happen.

**Static analyzers.** Clang Static Analyzer, Coverity, Infer. These tools run separately from compilation. They use techniques from program analysis (abstract interpretation, symbolic execution, path-sensitive analysis) to trace paths through the program and track state along branches. The analysis is expensive and must trade off precision against scalability. They use heuristics to reduce false positives, which means they also miss real bugs.

**Region annotations.** Research languages like Cyclone and ATS. Cyclone added region types to C: every pointer is annotated with the region it points into, and the compiler tracks region lifetimes. ATS uses dependent types and linear types for similar effect. Region inference can reduce the annotation burden. These languages achieve compile-time safety but never saw wide adoption. The annotation requirements and limited C compatibility kept these languages in research settings.

**Garbage collection.** Java, Go, Python, JavaScript, C#, Ruby. The problem we described is coordinates pointing to space that no longer exists. Garbage collection inverts the relationship: space is not freed until no coordinates point to it. The runtime periodically traces from roots (stack, globals) to find all reachable objects. Only unreachable objects are reclaimed. A coordinate cannot outlive its space because the space is kept alive as long as any coordinate reaches it. Common algorithms include mark-and-sweep, copying collectors, and generational collection. The tradeoff is runtime overhead: memory for the collector's bookkeeping, CPU time for tracing, and unpredictable pauses when collection runs.

**Ownership.** Rust. The language is designed so the compiler can build the complete graph and track where every coordinate points. Each value has a single owner. When ownership transfers, the previous binding becomes invalid. The analysis runs at compile time with zero runtime cost. The rest of this course explores how Rust makes this work.

<details>
<summary>Checkpoint</summary>
<p>You know the landscape: no analysis (C/C++), static analyzers (heuristics), region annotations (Cyclone/ATS), garbage collection (runtime tracking), ownership (Rust). You understand GC inverts the problem: space lives as long as coordinates reach it. You know Rust is designed so the compiler can build the complete graph.</p>
</details>

---

## How Rust Enables the Analysis

The previous sections described what a compiler would need to catch coherence failures, and why C++ cannot provide it. The compiler needs to answer two questions. Where does each coordinate point? When does each space become invalid? If the compiler can answer both, it can check whether any coordinate is used after its target space dies.

C++ hides both answers. Coordinates can be fabricated from integers, computed via arithmetic, type-erased through `void*`. The compiler cannot trace them back to a source. Heap space can be freed anywhere by anyone. The compiler cannot predict when space becomes invalid.

Rust makes both answers visible.

### Constrained Coordinates

In C++, a pointer is a number that happens to be used as an address. The language provides operations on numbers, and pointers inherit them. You can cast an integer to a pointer, perform arithmetic to compute new addresses, erase type information through `void*`, and store pointers in integer variables. These operations exist because pointers are numbers, and numbers support these operations.

```cpp
int* p = (int*)0x1234;           // coordinate from integer
int* q = p + 5;                  // coordinate from arithmetic
void* v = p;                     // type erased
uintptr_t n = (uintptr_t)p;      // stored as integer
```

Each of these operations severs the connection between the coordinate and its origin. The compiler saw `p` created from an integer literal, not from taking the address of a variable. It cannot know what space `p` points to. The compiler saw `q` computed from arithmetic, and the destination depends on runtime values. The compiler saw `v` assigned from `p` with its type erased, and now cannot know what type of space it refers to. The analysis requires knowing where every coordinate points, and C++ provides no such guarantee.

Rust references are not numbers. A reference is a typed relationship between a binding and a target. The language provides one way to create a reference, and that is to take the address of an existing value using the `&` operator. There is no syntax for creating a reference from an integer. There is no syntax for reference arithmetic. There is no type erasure for references.

```rust
let x = 5;
let r = &x;              // r points to x, compiler knows this
```

When the compiler sees `&x`, it records that the resulting reference points to `x`. Every reference in the program has a known origin. The compiler can trace any reference back to the space it refers to.

| C++ allows | Rust equivalent | Why it matters |
|------------|-----------------|----------------|
| `(int*)0x1234` | Does not exist | Every reference has a known target |
| `p + 5` | Does not exist | References cannot drift to unknown locations |
| `void* v = p` | Does not exist for references | Type information preserved |
| `(uintptr_t)p` | Does not exist for references | References cannot become integers |

Raw pointers in Rust (`*const T`, `*mut T`) allow some of these operations, but using a raw pointer requires `unsafe` code. The borrow checker analyzes safe Rust, where all coordinates are references with known origins.

### Heap Lifetime Control

Stack space has predictable lifetimes. A local variable lives from its declaration until the scope ends. The compiler sees scope boundaries in the source text and knows exactly when stack space dies.

Heap space in C++ has unpredictable lifetimes. A `new` expression allocates space, and that space lives until some code calls `delete`. The coordinate to that space can travel through the program, copied into other variables, passed to functions, stored in data structures. Any code holding the coordinate can free the space. Any code holding a copy of the coordinate can use it afterward and find the space gone. Nothing in the source text tells the compiler when heap space will die.

Rust makes heap lifetimes predictable by assigning exactly one owner to each allocation. The owner is a binding, and that binding has a scope. Rust deallocates the owned space when the owner's scope ends. Heap lifetime follows owner scope, the same way stack lifetime follows function scope.

```rust
fn example() {
    let v = vec![1, 2, 3];    // v owns heap space
    // ... use v ...
}                              // v's scope ends, heap space freed
```

The binding `v` owns the vector's heap allocation. When `v` goes out of scope, the heap space is freed. The compiler sees the scope boundary and knows when the space dies.

Ownership can transfer from one binding to another. This is a move. After the move, the original binding is no longer valid, and the new binding is the owner.

```rust
fn example() {
    let v = vec![1, 2, 3];    // v owns
    let w = v;                 // ownership moves to w, v now invalid
    // ... use w ...
}                              // w's scope ends, heap space freed
```

At any moment, exactly one binding owns each allocation. The compiler tracks ownership through assignments and function calls. When the owner's scope ends, the space dies. The compiler knows when that happens because it can see the scope.

This transforms the analysis. Without ownership, heap space can die at any point where code calls `free` or `delete`. The compiler would need to trace every path through the program to find all such points. With ownership, heap space dies at scope boundaries, and the compiler already sees those. Heap behaves like stack for the purposes of lifetime analysis.

### Separate Compilation

Constrained coordinates and ownership make both questions answerable within a single function. The compiler traces each `&` to its source and finds each owner's scope end. Both answers are visible in the function body.

Separate compilation limits this. The compiler processes each function independently. It sees the signature of each function it calls. It does not see the body. If a function returns a reference, the caller cannot trace through the implementation to find where the reference points.

Function signatures solve this. The signature declares, for each output, what kind of thing the caller receives.

**Owned output.** The function created new space and returns ownership to the caller.

```rust
fn create() -> Vec<i32> {
    vec![1, 2, 3]
}
```

The caller receives ownership. No connection to any input. The caller controls when this space dies.

**Value output.** The function returns a plain value. No reference, no coordinate.

```rust
fn length(list: &[i32]) -> usize {
    list.len()
}
```

A value is self-contained. Nothing to track.

**Borrowed output.** The function returns a reference that points into space owned by one of its inputs.

```rust
fn first(list: &[i32]) -> &i32 {
    &list[0]
}
```

The output points into `list`. The caller must keep `list` alive while using the output. This is the case where connection information matters. The signature must declare which inputs the output may borrow from.

A function cannot return a reference to its own local stack space, because that space dies when the function returns. The compiler rejects this as an error.

```rust
fn broken() -> &i32 {
    let x = 5;
    &x              // x dies at return, this would dangle
}
```

Static space is always valid and requires no tracking. These cases are straightforward and the compiler handles them without annotation.

The three output kinds cover what the caller needs to know. Owned and value outputs require no connection information. Borrowed outputs require the signature to declare which inputs the output connects to. This declaration is what Rust calls a lifetime annotation. The caller reads the signature instead of tracing the implementation. The compiler verifies each function independently, and the information needed for whole-program safety flows through signatures.

### Ownership Notation

The three output kinds determine what the caller's binding looks like. Rust's assignment syntax hides which kind applies. `let result = create()` and `let elem = first(&list)` look identical. One produces an owner. The other produces a name.

The language hides these distinctions deliberately. A learner building the model for the first time does not have the experience to see through the syntax, and learning from notation that conceals the distinctions being taught means working against the medium.

The course includes a companion crate called `notation` with a macro called `explicit!`. Inside the macro, every operation says what it does. This chapter uses `notation` as its primary syntax. Later chapters unpack the notation into standard Rust.

```rust
explicit! {
    let owner(s) = take(String::from("hello"));          // s takes ownership of new string
    let owner(t) = take(s);                              // t takes ownership from s, s invalid
    let name(r) = coord_exclusive(t);                    // r is a coordinate to t's space
    println!("{}", at(r));                                // follow the coordinate
}
```

The left side of each `let` declares the kind of binding. `owner` means the binding controls when space dies. `name` means the binding holds a coordinate to space owned elsewhere. The right side declares what happens. `take` means the binding takes ownership of the expression result. When the argument is an existing binding, that binding becomes invalid. When the argument is a constructor or literal, nothing else is affected. `coord_exclusive` creates an exclusive coordinate. `at` follows a coordinate to its target.

The vocabulary maps to the framework from Chapter 2. `owner` tracks SPACE x TIME. `name` tracks COORDINATES. `take` is an ownership transfer in TIME. `coord_exclusive` creates a COORDINATE with exclusive access. The macro makes the framework visible inside the syntax.

This chapter works with `owner`, `name`, `take`, `coord_exclusive`, and `at`. Coordinates come in two kinds. `coord_exclusive` grants read and write access. `coord_shared` grants read access and allows multiple coordinates to coexist. This chapter focuses on exclusive coordinates. `coord_shared` appears in some function signature examples, but the rules governing how the two kinds interact are the subject of the next chapter.

### Output to Input Mapping

A function signature declares whether the caller receives an `owner` or a `name` for each output. Owned and value outputs produce `owner` bindings that the caller controls independently. Borrowed outputs produce `name` bindings that point into space the caller already owns.

```
// Owned output: caller receives ownership
let owner(result) = create()

// Value output: caller receives a value
let owner(n) = length(coord_shared(list))

// Borrowed output: caller receives a coordinate into input space
let name(elem) = first(coord_shared(list))
```

For borrowed outputs, the signature encodes which input the `name` connects to. The caller must keep that input alive while the `name` exists. This is what lifetime annotations encode.

```
first(list: &[i32]) -> &i32
  caller: let name(o) = first(...)    // o borrows from list

search(map: &Map, key: &K) -> &V
  caller: let name(o) = search(...)   // o borrows from map

either(a: &i32, b: &i32, flag: bool) -> &i32
  caller: let name(o) = either(...)   // o borrows from a or b

create() -> Vec<i32>
  caller: let owner(o) = create()     // o owns new space

length(list: &[i32]) -> usize
  caller: let owner(o) = length(...)  // o is a value, self-contained
```

| Function | Output     | Borrows from | Caller keeps alive        |
| -------- | ---------- | ------------ | ------------------------- |
| `first`  | `name(o)`  | `list`       | `list`                    |
| `search` | `name(o)`  | `map`        | `map` (key not in output) |
| `either` | `name(o)`  | `a` or `b`   | both `a` and `b`          |
| `create` | `owner(o)` | nothing      | nothing (caller controls) |
| `length` | `owner(o)` | nothing      | nothing (self-contained)  |

This is the output-to-input mapping that signatures declare. The caller reads the signature instead of tracing the function body. Each function is verified independently, and the mapping flows through signatures. This solves the separate compilation problem from the previous section.

### Lifetime Annotations

Rust encodes the output-to-input mapping with lifetime annotations. Each `'a` is a label. The same label on an input and an output means "the output borrows from this input."

```rust
fn first<'a>(list: &'a [i32]) -> &'a i32
//                  ^^              ^^  same label: output borrows from list

fn search<'a, 'b>(map: &'a Map, key: &'b K) -> &'a V
//                      ^^                       ^^  output borrows from map, not key

fn either<'a>(a: &'a i32, b: &'a i32, flag: bool) -> &'a i32
//                ^^           ^^                      ^^  output borrows from both
```

The notation made the concept visible. The `'a` syntax is how Rust encodes it in the type system. Same information, different spelling.

### C++ Has No Equivalent

C++ programmers write functions that return references. The signatures look similar.

```cpp
const int& first(const vector<int>& list);
const V& search(const Map& map, const K& key);
const int& either(const int& a, const int& b, bool flag);
```

The C++ caller reads `const int& first(const vector<int>& list)` and knows the output is a reference. The signature does not say where it points. It might point into `list`. It might point to a static. It might point to a member variable of some object. The caller must read the implementation or trust documentation.

The Rust caller reads `fn first<'a>(list: &'a [i32]) -> &'a i32` and the same `'a` answers the question. The output points into `list`.

| C++ signature | Rust signature | What Rust adds |
|---|---|---|
| `const int& first(const vector<int>& list)` | `fn first<'a>(list: &'a [i32]) -> &'a i32` | Output borrows from `list` |
| `const V& search(const Map& map, const K& key)` | `fn search<'a, 'b>(map: &'a Map, key: &'b K) -> &'a V` | Output borrows from `map`, not `key` |
| `const int& either(const int& a, const int& b, bool flag)` | `fn either<'a>(a: &'a i32, b: &'a i32, flag: bool) -> &'a i32` | Output borrows from both |

### The Compiler Checks Both Sides

The signature is a contract. The compiler enforces it in two places independently.

At the call site, the compiler reads the signature and requires the caller to keep the borrowed inputs alive while the output exists. The compiler does not need the function body. A function pointer, a trait object, a function from another crate — the caller-side analysis works the same way, because the signature carries the mapping.

When the compiler does have the function body, it verifies that the implementation matches the signature. If the signature says the output borrows from `list`, the body must return a reference derived from `list`. Returning a reference to a local variable, or to the wrong input, is a compile error.

The two checks are independent. The caller-side analysis never needs the body. The body-side check never needs the call site. Lifetime annotations solve the separate compilation problem by encoding everything the caller needs into the signature.

<details>
<summary>Checkpoint</summary>
<p>You understand the compiler needs two answers: where each coordinate points, and when each space becomes invalid. You know Rust's design choices: constrained references (traceable origins) and ownership (heap lifetime tied to scope). You understand separate compilation is solved by signatures declaring output-to-input connections. You know the three output kinds (owned, value, borrowed) and see function outputs as <code>owner</code> or <code>name</code> bindings in the caller's scope. You know the <code>'a</code> syntax: the same label on an input and output means "the output borrows from this input." You understand the compiler checks both sides of the signature contract independently.</p>
</details>

---

## Using Data, Managing Space

The previous sections described two kinds of bindings. An `owner` controls when space dies. A coordinate lets code use the data in space owned elsewhere. Using the data and managing the space are different operations, and Rust enforces the difference.

A coordinate, even an exclusive one, lets code read and write the target data. It cannot free the space, move ownership out, or replace the allocation. Only the `owner` can manage the space.

In `notation` syntax, the distinction looks like this.

```rust
explicit! {
    let owner(b) = take(Box::new(String::from("hello")));  // b owns heap space
    let name(r) = coord_exclusive(b);                      // r can read and write
    // take(at(r)) would be invalid because r is a name, and names cannot own
    // the path to the heap goes through a coordinate, so ownership cannot transfer
}
```

The binding `b` is an owner. It controls when the string's heap space is deallocated. The binding `r` is a coordinate to `b`'s space. Through `r`, code can read the string and modify its contents. Through `r`, code cannot move the string to a new owner, because moving requires consuming the owner, and `r` is not the owner.

If `r` were the owner, moving would work.

```rust
explicit! {
    let owner(b) = take(Box::new(String::from("hello")));  // b owns heap space
    let owner(s) = take(b);                                // ownership transfers, b invalid
}
```

`take(b)` works because `b` is an `owner`. Ownership transfers to `s`, and `b` becomes invalid. The compiler forbids `take` through a `name`. Names let code use the data. Owners manage the space.

The distinction makes the compiler's analysis airtight. A function that receives a `name` parameter cannot `take` through it, cannot destroy the space, cannot move ownership away. The caller knows that after the function returns, the space still exists. The function used the data through the `name`. Managing the space stayed with the `owner`. Signatures that declare `name` connections are telling the truth in full, because a `name` cannot do more than use the data.

The control flow analysis from earlier in the chapter traced paths to find dead space access. The compiler marks where space dies and checks whether any path leads from death to access. The use-manage distinction simplifies this analysis at function boundaries.

```
let owner(s) = take(String::from("hello"))            // A
something(coord_exclusive(s))                          // B: passes a name
println!("{}", at(s))                                   // C: uses s
```

```
     ┌───┐
     │ A │ owner(s) created
     └─┬─┘
       ▼
     ┌───┐
     │ B │ something receives a name
     └─┬─┘
       ▼
     ┌───┐
     │ C │ at(s) — safe ✓
     └───┘
```

The compiler asks whether `s`'s space can die between A and C. The function `something` receives a `name`. Names cannot `take`. The space survives the call. The access at C reaches live space.

If the caller wrote `something(take(s))`, the function would receive an `owner`. The space might die inside the function body. The compiler would mark B as a potential death point and reject the access at C.

The compiler does not need to see inside `something`. The signature declares whether each parameter is an `owner` or a `name`, and that declaration determines whether the caller's space can die during the call. This connects the use-manage distinction to the separate compilation problem from the previous section.

### What C++ Allows

C++ programmers know `unique_ptr` as the closest analog to Rust's ownership model. A `unique_ptr` owns heap space, and when it goes out of scope, the space is freed. A reference to a `unique_ptr` allows access to the owned data. So far, the analogy holds.

The analogy breaks at space management. In C++, a reference to a `unique_ptr` can transfer ownership away from it.

The following table shows what a function can do through an exclusive coordinate to an owner. The C++ columns show both a reference (`unique_ptr<string>&`) and a pointer (`unique_ptr<string>*`) to the same unique_ptr. The Rust column uses `spelled` syntax for `&mut Box<String>`.

C++ references are syntactically transparent. Writing `*ref` already dereferences the unique_ptr, because the reference adds no visible layer. A pointer requires an explicit `*ptr` first, then another `*` through the unique_ptr. Rust's `at()` works like the pointer. Every coordinate is visible and requires an explicit `at()` to follow.

| Operation        | C++ via `&` ref              | C++ via `*` ptr                | Rust via `name(r) = coord_exclusive(b)` |
| ---------------- | ---------------------------- | ------------------------------ | --------------------------------------- |
| Read value       | `*ref`                       | `**ptr`                        | `at(at(r))`                             |
| Write value      | `*ref = v`                   | `**ptr = v`                    | `at(at(r)) = v`                         |
| Call methods     | `ref->method()`              | `(*ptr)->method()`             | `at(r).method()`                        |
| Move out         | `auto s = move(*ref)`        | `auto s = move(**ptr)`         | `take(at(r))` **forbidden**             |
| Destroy space    | `ref.reset()`                | `ptr->reset()`                 | **forbidden**                           |
| Replace contents | `ref = make_unique(v)`       | `*ptr = make_unique(v)`        | `mem::replace` (owner stays valid)      |

The first three rows are using the data. Both languages allow this through any coordinate. The last three rows are managing the space. They free it, move ownership out, or replace the allocation. C++ allows a coordinate to manage the space. Rust does not. A `name` can use the data. Only an `owner` can manage the space.

Passing by value transfers ownership in both languages. The receiver is the `owner` and can do anything. Shared coordinates (`const unique_ptr<T>&` in C++, `coord_shared` in Rust) restrict both languages to reading.

```cpp
void drain(std::unique_ptr<std::string>& ref) {
    auto stolen = std::move(*ref);   // ref now holds nullptr
    // caller's unique_ptr is gutted
}
```

The function received a reference. Through that reference, it called `std::move` on the contents and took the string. The caller's `unique_ptr` is now empty. A coordinate was used to exercise authority over the space.

In Rust, the equivalent is forbidden.

```rust
fn drain(r: &mut Box<String>) {
    let stolen = *r;    // ERROR: cannot move out of `*r`
}
```

The compiler rejects this. The binding `r` is a coordinate, and coordinates cannot move ownership out of the space they point to. The owner still holds authority. The function can read and write through `r`, but it cannot consume what `r` points to.

`std::mem::replace` and `std::mem::take` appear to contradict this, because they operate through `&mut` references and produce owned values. They do not contradict it. Both functions swap the contents of the target space with a replacement value. The owner always owns something valid after the operation. The coordinate cannot orphan or destroy the space. The value inside the space changes. The space itself continues to exist.

```rust
fn replace_contents(r: &mut String) -> String {
    std::mem::take(r)   // r now holds String::new(), caller's space still valid
}
```

The function took the string out and left an empty string in its place. The owner still owns valid space. The coordinate swapped the contents and left the space intact.

<details>
<summary>Checkpoint</summary>
<p>You understand coordinates let code use data, only owners manage space. Even exclusive coordinates cannot destroy space or transfer ownership. You know the C++ comparison: <code>std::move(*ref)</code> guts the owner through a reference, while Rust forbids it. You know <code>mem::take</code>/<code>mem::replace</code> are not exceptions because they swap and leave valid space. This makes the compiler's analysis airtight: signatures tell the full story because coordinates cannot exceed their role.</p>
</details>

---

<details>
<summary>Checkpoint</summary>
<p>The compiler needs two answers to catch coherence failures. Where does each coordinate point? When does each space become invalid? Rust makes both visible. Constrained references give every coordinate a traceable origin. Ownership ties heap lifetime to scope. Lifetime annotations encode the output-to-input mapping so the analysis works across function boundaries without seeing the implementation. Coordinates let code use data. Only owners manage space.</p>
</details>
