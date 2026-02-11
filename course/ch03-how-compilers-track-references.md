---
layout: course
title: Catching Coherence at Compile Time
short_title: Compiler Analysis
chapter: 3
permalink: /learn-rust/how-compilers-track-references/
---

# Catching Coherence at Compile Time

Chapter 1 showed bugs that Rust catches and C++ misses. Chapter 2 explained why these bugs exist: they are coherence failures between SPACE, TIME, and COORDINATES. Most involve coordinates pointing to space that no longer exists.

This chapter asks: how can a compiler catch these bugs? What would it need to track? Why doesn't C++ do this? And how does Rust make it possible?

<details>
<summary>Checkpoint</summary>
<p>Understands this chapter explains how a compiler could catch coordinate coherence failures. Motivated by the question: what would that require?</p>
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
<p>Sees the ch01 dangling reference analyzed in detail. Understands two bindings: x holds a value, r holds a COORDINATE. Understands that taking an address creates a coherence obligation between independent lifetimes.</p>
</details>

---

## Why You Need Coordinates

We use "coordinate" rather than "pointer" or "reference" because languages define those terms differently. In C++, `T*` is a pointer and `T&` is a reference. In Rust, `&T` is a reference and `*const T` is a raw pointer. The conceptual problem is identical: something that tells you where data lives. "Coordinate" abstracts over these language-specific terms.

Coordinates are not optional. Copying large data costs time and energy, so programs pass addresses instead of values. Data structures like graphs, trees, and linked lists require indirection because you cannot inline a cycle. Any language that cares about performance or expressiveness needs coordinates.

Coordinates are syntactically independent and semantically dependent. Syntactically independent means `r` has its own declaration, its own location, its own scope, and the compiler processes it separately from `x`. Semantically dependent means the purpose of `r` is to refer to `x`, and without `x`, the address in `r` points to nothing meaningful. One way that coherence problems arise is semantics that syntax does not capture.

<details>
<summary>Checkpoint</summary>
<p>Understands why we use "coordinate" as a term. Knows coordinates are unavoidable: physics (copying costs), data structures (indirection required). Understands coordinates are syntactically independent and semantically dependent. Knows coherence problems arise from semantics that syntax does not capture.</p>
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
       +---+
       | A | let x = 5
       +-+-+
         v
       +---+
       | B | r = &x
       +-+-+
         v
       +---+
       | C | x dies (scope ends)
       +-+-+
         v
       +---+
       | D | *r -- ACCESS with dead space
       +---+
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
                   +---+
                   | A | if flag
                   +-+-+
             +------+------+
             v             v
           +---+         +---+
           | B |         | C |
           |x=5|         |y=10|
           |r=&x|        |r=&y|
           +-+-+         | x | <-- y dies (scope ends)
             |           +-+-+
             v             v
           +---+         +---+
           | D |         | F |
           |*r |         | x | <-- propagated
           |safe|        +-+-+
           +-+-+           |
             |             |
             +------+------+
                    v
                  +---+
                  | G |
                  |*r | <-- ACCESS, unsafe on else path
                  +---+
```

The access at D is safe because no path from a death reaches it. The access at G is unsafe because a path from C reaches it through F. The compiler finds the unsafe path and rejects.

<details>
<summary>Checkpoint</summary>
<p>Understands source text has hierarchical structure that the RAM model lacks. Knows the compiler bridges them using intermediate representations like the control flow graph. Understands dead space detection as graph reachability: can any path lead from space death to coordinate access?</p>
</details>

---

## Memory Architecture and Compiler Analysis

The RAM model is flat, but programs organize memory into regions with different properties.

Static space lives for the entire program. Coordinates to static space are always valid.

Stack space dies when scopes end. The compiler sees scope boundaries in the source text.

Heap space dies when freed. Without constraints, the coordinate to heap space can travel through the program, and any code holding it can free it. No lexical structure governs heap. Why compilers cannot simply track this, and how Rust constrains it, are questions we address shortly.

```
STACK ONLY (tree)              HEAP ONLY (graph)           WHAT WE HAVE (hybrid)

    main                        +---+   +---+                 main
    +-- foo                     | A |<--| B |             +-- foo ------+
    |   +-- bar                 +-+-+   +---+             |   +-- bar   |
    |       +-- baz               |       ^               |             v
    +-- qux                       v       |               +-- qux    [heap]
                                +---+   +-+-+                          ^
                                | C |-->| D |                          |
                                +---+   +---+              ------------+

Coords point UP only.          Coords point ANYWHERE.      Stack: tree. Heap: escapes.
```

For dead space detection, the compiler can determine from the source text when stack coordinates are valid. Heap has no such property. The compiler cannot see when heap space dies.

The stack's lexical structure makes functions natural units for dead space detection.

<details>
<summary>Checkpoint</summary>
<p>Understands memory regions: static (always valid), stack (lexical structure visible), heap (no lexical structure). Knows the compiler can see when stack coordinates are valid but not heap. Understands that stack's lexical structure makes functions natural units for dead space detection.</p>
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

In the next chapter, we will see why even without these obstacles, perfect analysis is still impossible.

<details>
<summary>Checkpoint</summary>
<p>Understands why C++ compilers cannot do this analysis. Knows the three obstacles: separate compilation (incomplete graph), function pointers (unknown destinations), unconstrained pointers (coordinates from nowhere).</p>
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
<p>Knows the landscape: no analysis (C/C++), static analyzers (heuristics), region annotations (Cyclone/ATS), garbage collection (runtime tracking), ownership (Rust). Understands GC inverts the problem: space lives as long as coordinates reach it.</p>
</details>

---

## How Rust Enables the Analysis

The previous sections described two obstacles to compile-time coordinate analysis. Unconstrained pointers mean the compiler cannot track where coordinates point. Separate compilation means the compiler cannot see inside functions. Rust addresses both, and adds a third element that makes heap memory as predictable as stack memory.

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

Rust makes heap lifetimes predictable by assigning exactly one owner to each allocation. The owner is a binding, and that binding has a scope. When the owner's scope ends, the owned space is deallocated. Heap lifetime becomes tied to owner scope, the same way stack lifetime is tied to function scope.

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

### What Crosses Function Boundaries

Rust still has separate compilation. The compiler cannot see inside every function. The analysis must work without tracing paths through function bodies.

The question is what a caller needs to know. A function body operates on space, time, and coordinates. It may create local variables, allocate heap, take addresses, and return values. The caller does not see these operations. The caller needs to know the net effect on lifetimes and coordinates.

Consider what a function can output. The output can point to one of four places.

**Space from an input.** The function received a reference to space owned by the caller. The output might point into that space.

```rust
fn first(list: &[i32]) -> &i32 {
    &list[0]
}
```

The output points into `list`. The caller must keep `list` alive while using the output.

**New owned space.** The function allocated heap and returns ownership to the caller.

```rust
fn create() -> Vec<i32> {
    vec![1, 2, 3]
}
```

The output is new space. The caller receives ownership and controls when it dies. There is no connection to any input because the function created this space.

**Static space.** The function returns a reference to data that lives for the entire program.

```rust
fn greeting() -> &'static str {
    "hello"
}
```

The output points to static space. It is always valid. There is no connection to any input.

**Local stack space.** The function's local variables die when the function returns. A reference to local stack space would dangle.

```rust
fn broken() -> &i32 {
    let x = 5;
    &x              // x dies at return, this would dangle
}
```

The compiler rejects this. An output cannot point to local stack space because that space does not survive the return.

The four cases cover every possibility. Cases two and three require no tracking across function boundaries. Case four is an error the compiler catches. Case one is where connection information matters.

### Why Connections Are Sufficient

Ownership handles heap. The owner's scope determines when heap space dies. The compiler sees scope boundaries.

Connections handle borrowed outputs. When a function returns a reference that points into input space, the caller needs to know which input. The caller keeps that input alive while using the output.

Together they cover everything.

A function signature in Rust encodes both. Owned outputs are indicated by owned types in the return position. Borrowed outputs are indicated by reference types with lifetime annotations that specify which inputs they connect to.

The following examples use a notation `input <& output` to mean that the output may hold a reference into the input's space. This is not Rust syntax. Rust's actual lifetime annotation syntax is covered in later chapters.

| Function | Output kind | Connections |
|----------|-------------|-------------|
| `first(list) -> &i32` | borrowed | `list <& output` |
| `search(map, key) -> &V` | borrowed | `map <& output` |
| `either(a, b, flag) -> &i32` | borrowed | `a <& output`, `b <& output` |
| `create() -> Vec<i32>` | owned | none needed |
| `length(list) -> usize` | value | none needed |
| `greeting() -> &'static str` | static | none needed |

For `first`, the output points into `list`. The caller must keep `list` alive.

For `search`, the output points into `map`, not `key`. The key is used only for lookup.

For `either`, the output could come from either `a` or `b` depending on the flag. The caller must keep both alive.

For `create`, the output is new owned space. The caller receives ownership. No connection to track.

For `length`, the output is a plain value, not a reference. Nothing to track.

For `greeting`, the output points to static space. Always valid. No connection to track.

Connection information in signatures replaces path tracing through function bodies. The caller does not need to see the function's code. The signature declares which inputs the output may borrow from. The caller uses that information to ensure those inputs outlive the output. Complete coverage without complete visibility.

<details>
<summary>Checkpoint</summary>
<p>Understands how Rust makes compile-time coordinate analysis possible. Knows the constraints (constrained coordinates, heap lifetime control via ownership) and the encoding (connection information in function signatures replaces path tracing). Ready for lifetime syntax and mechanics in later chapters.</p>
</details>
