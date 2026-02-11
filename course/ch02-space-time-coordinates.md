---
layout: course
title: Space, Time, Coordinates
short_title: Space/Time/Coords
chapter: 2
permalink: /learn-rust/space-time-coordinates/
---

# Space, Time, Coordinates

The previous chapter showed Rust catching bugs that C++ misses. This chapter steps back to ask a deeper question. What are those bugs, exactly? Why do they exist at all?

These bugs are not accidents of language design. They are not quirks of C++. They appear in every language that allows mutable state and shared memory. They appear because of physics.

This chapter builds a framework for understanding memory bugs. The framework is abstract. It applies to CPU caches, databases, distributed systems, and programming languages alike. We start here because Rust's solution only makes sense once the problem is clear. The next chapters will connect this framework to Rust's specific rules. The framework will reappear throughout the course, linking what might otherwise seem like unrelated features.

If you want to understand not just what Rust does, but why it does it, this is the foundation.

<details>
<summary>Checkpoint</summary>
<p>Understands this chapter explains the fundamental problem. Knows the framework is abstract and applies broadly. Motivated to learn the foundation.</p>
</details>

---

## The Physics

Physics creates distance. Distance forces copies. Copies require coherence.

This pattern appears at every scale. CPU caches exist because RAM is far from the CPU. Database replicas exist because users are far from the primary database. Thread-local storage exists because shared memory requires synchronization.

Every copy creates a derived representation with a sync obligation.

| Layer | Source | Copy | Sync Strategy |
|-------|--------|------|---------------|
| CPU cache | RAM | L1/L2/L3 line | MESI protocol |
| Compiler | Memory | Register | Register allocation |
| Language | Original value | Reference/alias | Ownership, locks, GC |
| Thread | Shared heap | Thread-local | Mutex, channels, atomics |
| Process | Shared memory | Process-local | IPC, message passing |
| Database | Primary | Replica | Replication protocol |
| Network | Origin server | CDN edge | TTL, invalidation |

**MESI.** Hardware invalidates other cores' copies before a write completes.

**Register allocation.** The compiler ensures a value is in only one place, register or memory, at any point in the generated code.

**Ownership, locks, GC.** Ownership ensures one writer at a time. Locks serialize access at runtime. GC ensures copies are not freed while still referenced.

**Mutex, channels, atomics.** Mutexes prevent simultaneous access. Channels move data so only one thread has it. Atomics make single operations indivisible.

**IPC, message passing.** Data is copied between processes. No shared state, no coherence problem.

**Replication protocol.** Changes flow from primary to replicas in a defined order, so replicas converge to the same state.

**TTL, invalidation.** The system discards stale copies after a time limit or when the source signals a change.

<details>
<summary>Checkpoint</summary>
<p>Sees coherence as a universal problem. Understands physics creates distance, distance forces copies, copies require coherence. Sees the pattern across layers from CPU caches to distributed systems.</p>
</details>

---

## Three Primitives

Physics constrains computation along three axes. These constraints permeate into computer architecture, programming languages, and system design. The coherence problem reappears at each layer and must be resolved at each layer. Hardware solves it one way, operating systems another, languages another. The axes are the same.

**SPACE.** Where data lives. Stack, heap, register, cache line, disk, remote server. Memory hierarchy is a ladder of spaces with different latencies.

**TIME.** When things happen. Compile-time vs runtime. Sequential execution vs parallel. Scope entry to scope exit. A value's lifetime.

**COORDINATES.** How we refer to data. Names at compile-time, addresses at runtime, references, pointers. Coordinates tell you where space is. We use "coord" and "address" interchangeably. For our purposes, coords are always addresses.

State is not a primitive, but it is central to the coherence problem. State is data that changes over time. A value at an address that may differ on the next read. Stateless computation has no coherence problem because there is nothing to get out of sync. Most coherence problems involve state.

<details>
<summary>Checkpoint</summary>
<p>Knows the three axes: SPACE, TIME, COORDINATES. Understands state is central but not a primitive. Has vocabulary to describe memory bugs.</p>
</details>

---

## Bugs as Interaction Failures

The primitives are simple. Their interactions produce every category of memory bug.

**SPACE x TIME.** When memory exists. Allocation, deallocation, lifetimes, scope. When space is created and when it ends.

**SPACE x COORDINATES.** How many paths lead to memory. Aliasing, ownership, null. Whether one or many coordinates point to the same space.

**TIME x COORDINATES.** When a reference is valid. Scope, shadowing, drop order. Whether coordinates outlive what they point to.

Memory bugs are coherence failures between primitives.

| Bug | Primitives | Breakdown |
|-----|------------|-----------|
| Use-after-free | SPACE x TIME x COORDINATES | Use (COORDINATES) after (TIME) free (SPACE) |
| Dangling pointer | TIME x COORDINATES | Pointer (COORDINATES) dangles (TIME: target gone) |
| Double free | SPACE x TIME | Free (SPACE) double (TIME: twice) |
| Data race | SPACE x TIME x COORDINATES | Data (SPACE) race (TIME: parallel) via aliases (COORDINATES) |
| Buffer overflow | SPACE x COORDINATES | Buffer (SPACE) overflow (COORDINATES: exceed bounds) |
| Uninitialized read | SPACE x TIME x COORDINATES | Read (COORDINATES) uninitialized (TIME: before) value (SPACE) |
| Memory leak | SPACE x TIME | Memory (SPACE) leaked (TIME: outlives need) |

These are well-known bugs. We will not explain each bug in detail here. The point is to show how each bug maps to the primitives.

The following diagrams show each bug as a timeline. Space exists for a period. Coordinates point to it. The bug occurs when the relationship between primitives breaks down.

The diagrams use the following notation.

Timeline symbols:
- `████` — valid: space exists and coordinates are safe to use
- `░░░░` — invalid or at risk: space is gone or coordinates are stale
- `▓▓▓▓` — risk realized: the bug occurs here
- `>event<` — the problematic event

Operations:
- `p <- addr(x)` — assign the address of x to p
- `at_addr(p)` — access memory through p
- `free_at_addr(p)` — free memory through p

**Use-after-free.** Use (COORD) after (TIME) free (SPACE)

```
TIME        t0              t1              t2              t3
EVENTS      create heap     p <- addr(heap) free heap       >at_addr(p)<
────────────────────────────────────────────────────────────────────────
HEAP        ████████████████████████████████
COORD (p)                   ████████████████░░░░░░░░░░░░░░░░▓▓▓▓░░░░░░░░
                                                            >BUG<
```

---

**Dangling pointer.** Pointer (COORD) dangles (TIME: target gone)

```
TIME        t0              t1              t2              t3
EVENTS      create x        r <- addr(x)    scope ends      >at_addr(r)<
────────────────────────────────────────────────────────────────────────
SPACE (x)   ████████████████████████████████
COORD (r)                   ████████████████░░░░░░░░░░░░░░░░▓▓▓▓░░░░░░░░
                                                            >BUG<
```

---

**Double free.** Free (SPACE) double (TIME: twice)

```
TIME        t0              t1              t2              t3
EVENTS      create heap     p <- addr(heap) free_at_addr(p) >free_at_addr(p)<
────────────────────────────────────────────────────────────────────────
HEAP        ████████████████████████████████
COORD (p)                   ████████████████░░░░░░░░░░░░░░░░▓▓▓▓░░░░░░░░
                                                            >BUG<
```

---

**Data race.** Data (SPACE) race (TIME: parallel) via aliases (COORD)

```
TIME        t0              t1              t2
EVENTS      create x        p <- addr(x)    >write_at_addr(p)<
                            q <- addr(x)    >write_at_addr(q)<
────────────────────────────────────────────────────────────────────────
SPACE (x)   ████████████████████████████████▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░
COORD (p)                   ████████████████████████████████████████████
COORD (q)                   ████████████████████████████████████████████
                                            >BUG< (parallel writes)
```

---

**Buffer overflow.** Buffer (SPACE) overflow (COORD: exceeds bounds)

```
TIME        t0              t1
EVENTS      create buf[5]   >at_addr(buf[7])<
────────────────────────────────────────────────────────────
SPACE       ████████████████████████████████ [0..4]
COORD [7]                   ▓▓▓▓ (no SPACE here)
                            >BUG<
```

---

**Uninitialized read.** Read (COORD) uninitialized (TIME: before) value (SPACE)

```
TIME        t0                      t1
EVENTS      declare x, p <- addr(x) >at_addr(p)<
────────────────────────────────────────────────────────────
SPACE (x)   ░░░░░░░░░░░░░░░░░░░░░░░░▓▓▓▓░░░░░░░░░░░░░░░░░░░░
COORD (p)   ████████████████████████████████████████████████
                                    >BUG< (SPACE has no value)
```

---

**Memory leak.** Memory (SPACE) leaked (TIME: outlives COORD)

```
TIME        t0              t1              t2
EVENTS      create heap     p <- addr(heap) >p scope ends<
────────────────────────────────────────────────────────────────────────
HEAP        ████████████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░->
COORD (p)                   ████████████████
                                            >BUG< (ability to free heap through p is lost)
```

<details>
<summary>Checkpoint</summary>
<p>Sees each bug as a breakdown between primitives. Understands the diagrams. Recognizes these bugs from experience.</p>
</details>

---

## Paradigms and Languages

Different paradigms solve the coherence problem differently.

| Language | Paradigm | Coherence Strategy |
|----------|----------|-------------------|
| Haskell | Functional | No mutation, no problem |
| Erlang | Actor | Process isolation, message passing |
| Clojure | Functional | Persistent data structures |
| Rust | Linear/ownership | Compile-time proof |
| Go | Imperative | Channels or locks (programmer chooses) |
| Java | OOP | Locks, volatile (programmer responsibility) |
| C/C++ | Imperative | Programmer responsibility |

**Haskell.** Functions cannot modify state. Data structures are immutable. Operations return new structures. Nothing changes, so nothing gets out of sync.

**Erlang.** Processes share no memory. Communication happens by copying messages between them. Each process owns its data exclusively.

**Clojure.** Data structures appear mutable. They are immutable. Modifying a map returns a new map that shares structure with the old one. Explicit constructs like atoms and refs handle actual state changes.

**Rust.** Each value has one owner. Borrowing rules ensure one writer or many readers, never both. Enforced at compile time with no runtime cost.

**Go.** Goroutines share memory. Channels transfer data between them. Mutexes protect shared access. The programmer chooses which to use.

**Java.** Objects live on a shared heap, accessible from any thread. Locks and volatile exist but are not required. Correct synchronization is the programmer's job.

**C/C++.** Any pointer can alias any memory. Any thread can write anywhere. No compiler enforcement. Coherence is entirely manual.

<details>
<summary>Checkpoint</summary>
<p>Sees how different languages approach coherence. Understands the spectrum from "eliminate the problem" to "programmer responsibility."</p>
</details>

---

## Features as Interaction Solutions

Languages solve coherence problems with specific features. Each feature targets a specific interaction between primitives. This section catalogs the features by which interaction they solve. Many features appear in multiple languages. Some are ancient. Some are recent. All address the same underlying physics.

**SPACE x TIME.** When memory exists. These features control allocation and deallocation. The goal is to ensure memory exists when needed and is freed when done.

**Garbage collection.** Invented for Lisp in 1959. The runtime tracks which memory is still reachable and reclaims the rest automatically. The programmer never frees manually. Use-after-free and double-free become impossible. The cost is unpredictable pauses and runtime overhead. Used by Java, Go, Python, JavaScript, and most modern languages.

**RAII / destructors.** Introduced in C++ in the 1980s. Memory lifetime is tied to scope. When a variable goes out of scope, its destructor runs and frees resources. Deterministic and predictable. No runtime tracking. Used by C++ and Rust.

**Reference counting.** Dates to the early 1960s. Each allocation tracks how many references point to it. When the count reaches zero, memory is freed. Deterministic like RAII. Cycles are a problem and require weak references or cycle detection. Used by Swift, Objective-C, Python (combined with GC), Rust (Rc/Arc).

**Stack allocation with lexical scope.** Dates to ALGOL in 1958. Memory lives on the stack and is automatically reclaimed when the scope ends. Fast and simple. No heap fragmentation. Size must be known at compile time. Universal in compiled languages.

**SPACE x COORDINATES.** How many paths lead to memory. These features control aliasing. The goal is to limit how many references can reach the same data.

**Ownership / move semantics.** Formalized in C++11, central to Rust. Each value has exactly one owner. Passing a value transfers ownership. The original binding becomes invalid. Aliasing is eliminated by default. Copying must be explicit.

**Value types.** Ancient, present in most languages. Data is copied rather than referenced. Each copy is independent. No aliasing because each copy is separate data. Copying large structures is expensive. Used for primitives in nearly all languages, for all types in some (Go structs, Swift structs).

**Nullable types / Option.** ML languages introduced this in the 1970s. References must be explicitly nullable. Code must handle the "no data here" case to compile. Null pointer dereferences become impossible. Used by ML, Haskell, Rust, Swift, Kotlin.

**TIME x COORDINATES.** When a reference is valid. These features control reference validity. The goal is to ensure references do not outlive what they point to.

**Lexical scope.** Dates to ALGOL in 1958. References are valid only within their declaring scope. Compiler rejects code that uses references outside their scope. Simple and effective for local variables.

**Lifetime annotations.** Introduced in Rust in 2010. Explicit markers that tell the compiler how long references live. Enables the compiler to verify references do not outlive their data. No runtime cost. Requires programmer annotation in complex cases.

**Closures capturing environment.** Dates to Lisp in 1958. When a closure captures variables, the language must decide what happens to them. Ownership and borrowing rules determine whether the closure copies, borrows, or moves the captured variables.

**Drop order guarantees.** Formalized in C++ and Rust. Destructors run in a defined order, reverse of declaration order. Prevents references to already-dropped values. Deterministic and predictable.

**SPACE x TIME x COORDINATES.** The full problem. When all three primitives interact, there is no elegant solution. Multiple threads accessing shared mutable state is fundamentally hard. The features in this category are trade-offs. Mutexes block. Atomics are limited. Channels require restructuring code. Immutability forbids mutation. Each gives up something to gain coherence.

**Mutex / RwLock.** Dates to the 1960s. Runtime enforcement of one writer or many readers. Threads block until they can acquire the lock. Simple to understand. Deadlocks are possible. Performance suffers under contention.

**Atomics.** Hardware support dates to the 1970s. Hardware-level indivisible operations. Single reads and writes cannot be interrupted, enabling lock-free coordination. Limited to simple operations. Correct usage is difficult.

**Channels / message passing.** Dates to CSP in 1978. Data is sent between threads by transferring ownership. The sender loses access, the receiver gains it. Requires restructuring code around message flow.

**Immutability.** Ancient idea, central to functional programming. If data cannot change, any number of readers are safe. No mutation means no write-read conflicts. Requires different programming style.

**Actor model.** Introduced in 1973. Each actor owns its state and communicates via messages. No shared state between actors. Used by Erlang, Akka, and others.

**Linear types.** Theoretical foundation dates to the 1980s. Each value must be used exactly once. Compiler enforces that resources are neither leaked nor used twice. Rust's ownership is a practical form of this.

**Rust borrow checker.** Introduced in Rust in 2010. Enforces one writer or many readers at compile time. Rejects programs that violate this rule before they run. No runtime cost. Requires learning new patterns.

<details>
<summary>Checkpoint</summary>
<p>Has a catalog of features organized by which primitive interaction they solve. Knows the historical context. Understands trade-offs. Has a framework for understanding memory bugs. Knows the vocabulary. Ready to see how Rust applies these ideas.</p>
</details>
