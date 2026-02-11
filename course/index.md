---
layout: course
title: Learn Rust
permalink: /learn-rust/
---

# Learn Rust

<p class="subtitle">A mental model for Rust's memory system — for programmers who already know C++.</p>

I cannot memorize rules. Never could. Give me twenty rules and they are gone in a week. Give me the system that produces those rules and I will remember it for years, because systems have structure and structure is compressible in a way that lists are not.

Rust tutorials gave me rules. "One mutable reference or many immutable." "Moves invalidate the source." "Lifetimes must not outlive their referent." Rules without the system underneath are needless pedantic abstractions. They tell you what the compiler will do in situations you have already seen. They tell you nothing about situations you haven't.

Stack architecture, computability constraints, Rice's theorem. We all know these things. They sit underneath the borrow checker and they explain every decision it makes. Dig into them and the rules stop requiring memorization because they become derivable. The borrow checker turns into a consequence of what memory is, what compilers can analyze, and what tradeoffs Rust chose.

That understanding did not come from any existing course. [The Rust Book](https://doc.rust-lang.org/book/), [Google's Comprehensive Rust](https://google.github.io/comprehensive-rust/), [r4cppp](https://github.com/nrc/r4cppp), [Codecademy](https://www.codecademy.com/learn/learn-rust), [Udemy](https://www.udemy.com/topic/rust/) — all great resources, all teaching the rules as axioms. This course does something different. SPACE x TIME x COORDINATES is a framework for memory bugs. Every use-after-free, every data race, every dangling reference is a failure in one of these three dimensions. The course builds this framework first, and the borrow checker falls out of it.

### Who it's for

Experienced programmers, particularly those with a C++ background, who already have a mental model of systems programming. The job of this course is to update that model, not build one from scratch.

If you are new to programming or new to systems languages, start with [The Rust Book](https://doc.rust-lang.org/book/). It builds the mental model this course assumes you already have. Come back here when you want to understand the system underneath the rules.

---

## Chapters

<ul class="chapter-list">
  <li>
    <a href="/learn-rust/first-contact/">
      <span class="ch-num">01</span> First Contact
      <span class="ch-desc">Before: Experienced C++ programmer. Has not written Rust.<br>After: Knows Rust catches bugs C++ misses. Does not know how yet. Curious.</span>
    </a>
  </li>
  <li>
    <a href="/learn-rust/space-time-coordinates/">
      <span class="ch-num">02</span> Space, Time, Coordinates
      <span class="ch-desc">Before: Knows Rust catches bugs C++ misses. Does not know how.<br>After: Has a framework for understanding memory bugs. Ready to see how Rust applies these ideas.</span>
    </a>
  </li>
  <li>
    <a href="/learn-rust/how-compilers-track-references/">
      <span class="ch-num">03</span> Catching Coherence at Compile Time
      <span class="ch-desc">Before: Has framework: SPACE, TIME, COORDINATES. Does not know how a compiler could detect these problems.<br>After: Understands how Rust makes compile-time coordinate analysis possible. Ready for lifetime syntax and mechanics.</span>
    </a>
  </li>
</ul>

---

## [Exercises](/learn-rust/exercises/)

The course includes hands-on exercises organized around the framework, not language features. Starts with the `explicit!` macro — a companion crate that makes Rust's implicit operations visible.
