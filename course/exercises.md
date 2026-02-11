---
layout: course
title: Exercises
nav_title: Exercises
permalink: /learn-rust/exercises/
---

# Exercises

## How these are organized

Most Rust courses organize exercises around language features. Structs, then enums, then traits, then generics, then lifetimes, then concurrency. The progression follows the language's syntax, from simple constructs to complex ones.

These exercises are organized around the framework. The progression follows the concepts — space, time, coordinates, coherence — and introduces language features as they become necessary to express each idea. You will encounter threads, mutexes, and memory ordering long before you would in a conventional Rust course, because the framework demands them early.

The result is a different shape. Declarations come first because they are the vocabulary. Foundations come second because they connect the vocabulary to the problems Rust exists to solve. Ownership space, interior mutability, lifetimes, and memory ordering each explore one region of the design space in depth. Synthesis ties them together.

## The sections

**0 · Declarations.** The `explicit!` notation and its mapping to real Rust. Every combination of `owner`/`name`, `take`/`mem_copy`/`coord_shared`/`coord_exclusive`, and `rebindable`. Vocabulary building. 21 exercises.

**1 · Foundations.** Derived data, the borrow rule, identity and validity, shadowing, move semantics, runtime coherence, threads, mutexes, memory ordering, and language design choices. The problems Rust exists to solve, seen from first principles. 10 exercises.

**2 · Ownership Space.** Where values live. `const` vs `static`, stack vs heap, `Box`, `Rc`, `RefCell`, `Arc`, `Mutex`, `Weak`, drop order, and memory layout. The spatial dimension of the framework. 10 exercises.

**3 · Interior Mutability.** `Cell`, `RefCell`, `Mutex`, `RwLock`, and the spectrum between compile-time and runtime enforcement. The borrow rule does not disappear when you need shared mutation. It moves to runtime. 10 exercises.

**4 · Lifetimes.** Validity, multiple inputs, elision rules, lifetimes in structs, `'static`, non-lexical lifetimes, value tracking, conservative analysis, restructuring code, and `unsafe` as an escape hatch. The temporal dimension of the framework. 10 exercises.

**5 · Memory Ordering.** Visibility, store buffers, release/acquire, `SeqCst`, `Relaxed`, spinlocks, double-checked locking, hardware models, acquire-release pairs, and lock-free queues. The coordinate dimension under concurrency. 10 exercises.

**6 · Synthesis.** Coherence strategies, weak reference cycles, thread pools, choosing between type strategies, implementing `Rc` from scratch, message passing vs shared state, the observer pattern, a lock-free stack, analyzing a real crate, and designing your own ownership scheme. Putting it all together. 10 exercises.

---

## 0 · Declarations

Rust hides a lot behind simple syntax. `let y = x` might copy the value, move it, or borrow it. The type determines which one. The programmer is expected to know, and the syntax offers no help.

This is a design choice. Experienced Rust programmers carry the distinctions in their head. A learner does not have those distinctions yet, and learning them from syntax that hides them is working uphill.

The course includes a companion crate called `spelled` with a macro called `explicit!`. It expands Rust's implicit syntax into a form where every operation says what it does.

```rust
explicit! {
    let owner(y) = take_or_mem_copy(x);
}
```

This compiles and runs. It does the same thing as `let y = x`, but it says what is happening.

### The left side

The left side of a `let` says what the binding receives.

`owner(y)` means y receives ownership. It controls when the space dies.

`name(r)` means r receives coordinates to space owned elsewhere. A reference.

Adding `rebindable()` means the binding can be pointed at something else later.

### The right side

The right side says what happens to the source.

`take(x)` transfers ownership. x becomes invalid.

`mem_copy(x)` duplicates the bytes. x stays valid.

`take_or_mem_copy(x)` lets the compiler decide based on the type. Copy types get copied, everything else gets moved.

`coord_shared(x)` creates shared coordinates. Multiple allowed at the same time.

`coord_exclusive(x)` creates exclusive coordinates. Only one allowed.

`at(r)` follows coordinates to get the value.

### The exercises

Each exercise shows the explicit version first, then asks you to write the real Rust equivalent. The explicit version makes the model visible. The real version hides it again. The gap between the two is what you are learning.

> **This chapter is a work in progress.** The notation and exercises are functional. The full walkthrough is still being written.
