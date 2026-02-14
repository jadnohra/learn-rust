---
layout: course
title: Limits of Compile-Time Analysis
short_title: Analysis Limits
chapter: 4
permalink: /learn-rust/limits-of-compile-time-analysis/
---

# Limits of Compile-Time Analysis

<details class="toc">
<summary>Contents</summary>

- [Where Branches Break the Analysis](#where-branches-break-the-analysis)
- [Why Perfect Analysis Is Impossible](#why-perfect-analysis-is-impossible)
- [Sound or Complete](#sound-or-complete)
- [What the Compiler Can Distinguish](#what-the-compiler-can-distinguish)
- [Why Compilers Skip Decidable Cases](#why-compilers-skip-decidable-cases)
- [Rust-Specific Approximation: Signatures](#rust-specific-approximation-signatures)
- [Escape Hatches](#escape-hatches)

</details>

Chapter 3 showed how Rust's compiler traces COORDINATES through control flow graphs to detect dead SPACE access. The analysis works for straight-line code, where every statement executes in sequence. Branches change this.

## Where Branches Break the Analysis

The control flow graph from chapter 3 traces all paths through a program. The compiler marks where SPACE dies and checks whether any path leads from death to access through a COORDINATE. For straight-line code, every path executes.

Branches create paths that may or may not execute, depending on runtime values.

```
r = uninitialized
{
    x = 5
    if some_condition(n) {
        r = &x
    }
}
// x is gone here
if some_condition(n) {
    print(*r)      // r used after x dies?
}
```

Whether `*r` accesses dead SPACE depends on whether `some_condition(n)` returns true. The compiler would need to evaluate `some_condition(n)` to know. That evaluation depends on `n`, and `some_condition` might contain arbitrary computation.

The Collatz sequence illustrates why arbitrary computation resists static prediction.

```
n = input
while n != 1 {
    if n is even { n = n / 2 }
    else { n = 3*n + 1 }
}
```

Whether this terminates for all inputs remains unknown. Mathematicians have tested every number up to 10^20 and the sequence always reaches 1, but no proof exists. If `some_condition(n)` checked whether Collatz reaches 1, the compiler would need to solve an open problem in mathematics to determine which path executes.

Newton's method for finding roots has the same property.

```
x = initial_guess
while |f(x)| > epsilon {
    x = x - f(x) / f'(x)
}
```

Convergence depends on the function and starting point. For some inputs the method converges, for some it cycles forever, for some it diverges. No general prediction is possible.

<details>
<summary>Checkpoint</summary>
<p>You see that branches create paths the compiler cannot resolve without evaluating runtime values. The Collatz sequence and Newton's method show you why arbitrary computation resists static prediction. You understand the compiler cannot determine which paths execute.</p>
</details>

---

## Why Perfect Analysis Is Impossible

Mathematical difficulty does not explain the impossibility. Turing proved something stronger in 1936.

A termination checker must return an answer for any program it examines. The program being examined has no such obligation. It can run forever, halt immediately, or do anything in between. A program can also read the checker's source code, call the checker on itself, and then do the opposite of what the checker predicts. If the checker predicts "terminates," the program loops forever. If the checker predicts "loops forever," the program terminates. The checker is wrong on this input regardless of what it answers. No algorithm can avoid this contradiction.

This is the Halting Problem. The technical term for such problems is undecidable. No algorithm exists that answers correctly for all inputs.

Rice's Theorem (1951) generalizes the result. Any non-trivial semantic property of programs is undecidable. "Non-trivial" means some programs have the property and some do not. "Semantic" means the property depends on what the program does, not how it is written. "Does this COORDINATE reach dead SPACE?" is such a property. So is "does this program have a data race?" and "does this program dereference a dangling pointer?"

These are mathematical impossibilities. They apply to any static analysis tool, in any language, built by any team.

<details>
<summary>Checkpoint</summary>
<p>You understand the Halting Problem and its diagonal argument. Rice's Theorem generalizes the result to all non-trivial semantic properties. You see that "does this COORDINATE reach dead SPACE?" is undecidable. These are mathematical impossibilities, not tooling gaps.</p>
</details>

---

## Sound or Complete

Perfect analysis is impossible, so the compiler must choose between two failures. It can accept some unsafe programs, or it can reject some safe ones.

**Sound** means the compiler never accepts unsafe code. It may reject code that is safe.

**Complete** means the compiler never rejects safe code. It may accept code that is unsafe.

No compile-time analysis can be both sound and complete for questions about program behavior. Rust chooses soundness. The asymmetry between failure modes makes this the right tradeoff. Accepting unsafe code means security vulnerabilities and undefined behavior. Rejecting safe code means restructuring or using escape hatches.

Ownership tracking aligns with how most programs are structured. Most safe code passes the compiler's analysis. The restrictions feel limiting at first but rarely block real work.

<details>
<summary>Checkpoint</summary>
<p>You see the compiler must choose between accepting unsafe code and rejecting safe code. You know soundness (never accept unsafe) and completeness (never reject safe). You understand Rust chooses soundness because the failure modes are asymmetric.</p>
</details>

---

## What the Compiler Can Distinguish

The analysis tracks COORDINATES. The granularity of that tracking determines which simultaneous accesses the compiler can prove safe.

**Struct fields.** `p.x` and `p.y` are distinct. The type declaration places them at distinct offsets, and field names are literal in source text. The compiler can prove that COORDINATES to `p.x` and COORDINATES to `p.y` point to disjoint SPACE.

**Array elements with literal indices.** `v[0]` and `v[1]` refer to different elements. The literal indices are visible in source text, so the compiler could distinguish them. Rust treats all index expressions as potentially overlapping. This is a design choice, covered in the next section.

**Array elements with computed indices.** `v[i]` and `v[j]` where `i` and `j` come from computation. Whether `i == j` depends on runtime values. Proving it is undecidable.

| COORDINATES to | Distinguishable? | Why |
|---|---|---|
| `p.x` vs `p.y` | Yes | Field names literal in source |
| `v[0]` vs `v[1]` | Could be | Rust's design choice |
| `v[i]` vs `v[j]` | No | May require solving halting problem |

Branches and pointer aliasing share this structure. Whether `*p` and `*q` point to the same SPACE depends on runtime values. Whether the same branch executes twice depends on runtime values. The compiler assumes worst case for both.

<details>
<summary>Checkpoint</summary>
<p>You understand the granularity of COORDINATE tracking. Struct fields are distinguishable. Literal array indices could be but are not. Computed array indices are undecidable. You see that branches and pointer aliasing share the same structure. The compiler assumes worst case for both.</p>
</details>

---

## Why Compilers Skip Decidable Cases

The literal index `v[0]` is decidable. So is a function call `pick(&a, &b, true)` where the argument is a constant. A human sees which branch executes. The compiler could too, using constant propagation and inlining.

General-purpose compilers prioritize predictability over precision. Code that compiles with `true` but fails with `some_function()` confuses users. Identifying which expressions fall into decidable subsets is itself expensive, and coverage is narrow. Specialized compilers do analyze decidable cases. Fortran compilers use polyhedral analysis for loop parallelization. ML frameworks require static tensor shapes. The techniques exist and work for specific domains.

Rust chose consistent approximate analysis. The compiler treats all index expressions and all branch conditions the same way. A program that compiles with a constant will still compile when that constant becomes a variable.

<details>
<summary>Checkpoint</summary>
<p>You understand that general-purpose compilers prioritize predictability over precision. Specialized compilers (Fortran, ML frameworks) analyze decidable subsets. Rust chose consistent approximate analysis. A program that compiles with a constant still compiles when that constant becomes a variable.</p>
</details>

---

## Rust-Specific Approximation: Signatures

Chapter 3 showed that function signatures declare which outputs borrow from which inputs. This solves the separate compilation problem. It also introduces an approximation.

Inside a function body, the compiler sees field-level COORDINATE access. `&d.field1` and `&d.field2` point to disjoint SPACE, and the compiler knows it.

```rust
let r1 = &d.field1;      // COORDINATES to d.field1
let r2 = &d.field2;      // COORDINATES to d.field2
                          // disjoint: yes
```

Across a function call, only the signature is visible. A method that borrows `self` borrows the whole struct.

```rust
let r1 = d.get_field1(); // signature says: borrows d
let r2 = d.get_field2(); // signature says: borrows d
                          // disjoint: the compiler cannot tell
```

Signatures encode COORDINATE connections at struct granularity. Field-level information does not cross function boundaries. Types describe whole structs, and connections are encoded through types.

These are the fundamental and Rust-specific limits. The remaining question is what to do when they cause the compiler to reject valid code.

<details>
<summary>Checkpoint</summary>
<p>You know signatures encode COORDINATE connections at struct granularity. Field-level information does not cross function boundaries. You connect this to ch03's signature mechanism.</p>
</details>

---

## Escape Hatches

Rust's soundness means rejecting some valid programs. The responses form a progression from least to most powerful.

### Restructure the Code

The signature-level approximation from the previous section means the compiler sometimes cannot tell which input a borrowed output connects to.

```
fn pick(a: &[i32], b: &[i32], cond: bool) -> &i32 {
    if cond { &a[0] } else { &b[0] }
}

let a = vec![1, 2, 3];
let b = vec![4, 5, 6];
let r = pick(&a, &b, true);
drop(b);
println!("{}", r);
```

The caller knows `r` comes from `a` because the argument is `true`. The signature declares that `r` might borrow from either `a` or `b`. The compiler assumes `b` might be needed and rejects the `drop(b)` while `r` exists.

The fix is structural. Move the `drop` after the last use of `r`, or call a function that takes only the input the caller needs. The code was safe, but the compiler could not verify it through the signature. Restructuring makes the safety visible.

### Encode Invariants in Types

The compiler cannot prove that `v[0..n]` and `v[n..len]` are disjoint SPACE, because index expressions depend on runtime values. A human sees the disjointness immediately. One range ends where the other begins.

```rust
let (left, right) = v.split_at_mut(n);
// left = v[0..n], right = v[n..len]
// disjoint by construction
```

Inside `split_at_mut`, an `unsafe` block asserts what the compiler cannot verify. The function computes two slices that are disjoint by construction, and the return type encodes the disjointness. The caller receives a safe interface. The proof obligation moved from the compiler to the function author.

This pattern generalizes. When you can prove a property the compiler cannot verify, build a function that uses `unsafe` internally and exposes a safe interface. Rust's standard library uses this pattern throughout.

### Move Verification to Runtime

Static analysis faces undecidability because it must answer questions about all possible executions. At runtime, only one execution happens. Questions that were undecidable statically become trivial runtime checks.

```rust
// The compiler rejects this: two mutable borrows of the same data
let r1 = &mut v[0];
let r2 = &mut v[1];
```

The compiler treats both index expressions as COORDINATES to the whole vector. It cannot prove they point to disjoint SPACE. The analysis is sound but overly conservative here.

`RefCell` moves the check to runtime. The program tracks how many COORDINATES are active and panics if the rules are violated during execution.

```rust
use std::cell::RefCell;

let v = RefCell::new(vec![1, 2, 3]);
let r1 = v.borrow_mut();  // runtime check: no other borrows active
```

Rust provides several types that shift verification from compile-time to runtime. The table maps each to its C++ equivalent.

| Rust | C++ equivalent | What it tracks |
|---|---|---|
| `RefCell<T>` | -- | Active borrow count |
| `Mutex<T>` | `std::mutex` + `T` | Lock state |
| `RwLock<T>` | `std::shared_mutex` + `T` | Reader/writer count |
| `Rc<T>` | `std::shared_ptr<T>` | Reference count |
| `Arc<T>` | `std::shared_ptr<T>` (atomic) | Reference count |

The tradeoff is runtime cost instead of compile-time rejection. The SPACE/TIME/COORDINATES analysis moves from source text to runtime state.

### Cyclic Structures

Ownership is tree-shaped. Each value has one owner, forming a hierarchy. Trees, DAGs, and acyclic structures fit this model. Cyclic structures do not.

```
    ┌───────────┐
    │  Node A   │
    │  next ──────────► Node B
    │  ◄────────────── prev  │
    └───────────┘    └───────────┘
```

A doubly-linked list node has `prev` and `next` COORDINATES to the same neighbor. Both cannot own the neighbor. If one owns and the other borrows, the borrowing COORDINATE cannot outlive the owning binding, and the structure cannot be rearranged freely.

Determining whether a node in a runtime-constructed graph is still reachable requires tracing the graph. This is the same problem garbage collectors solve. Static analysis cannot trace a runtime-constructed graph.

Rust chose compile-time ownership over runtime tracing. The cost surfaces when cycles are needed. The options are reference counting (`Rc` + `RefCell`), arenas that use indices instead of COORDINATES, or manual management with `unsafe`.

<details>
<summary>Checkpoint</summary>
<p>You know the four levels of escape hatches. Restructure the code when the signature approximation causes false rejection. Encode invariants in types using unsafe internally and safe interfaces externally (the split_at_mut pattern). Move verification to runtime (RefCell, Mutex, RwLock, Rc, Arc) when static analysis is too conservative. Cyclic structures do not fit tree-shaped ownership — the options are Rc + RefCell, arenas, or unsafe.</p>
</details>

---

## Scope of This Chapter

The compiler answers chapter 3's two questions with a sound approximation. Where do COORDINATES point? Trace the references. When does SPACE become invalid? Find the owner's scope end. The approximation rejects some valid programs. Escape hatches handle each case. Restructure the code, encode invariants in types, move verification to runtime, or assert correctness with `unsafe`.

The analysis so far has used `coord_exclusive`, one COORDINATE to one SPACE at a time. No aliasing. The next chapter introduces `coord_shared`, where multiple COORDINATES to the same SPACE coexist. Multiple readers are safe. A reader and a writer are not. The compiler needs a new rule for that.

<details>
<summary>Checkpoint</summary>
<p>You understand why perfect analysis is impossible, why Rust chooses soundness, and what to do when the compiler rejects valid code. You are ready for shared coordinates and the aliasing rule.</p>
</details>
