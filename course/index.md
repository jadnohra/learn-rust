---
layout: course
title: Learn Rust
permalink: /learn-rust/
description: "Derive the borrow checker from first principles. A course on Rust's memory model for C++ programmers — SPACE × TIME × COORDINATES."
image: /assets/img/learn-rust-og.png
---

# <i class="fab fa-rust"></i> Learn Rust

This course exists because I cannot memorize rules and never could. If I understand the system that produces the rules, I remember it for years, because systems have structure and structure compresses.

[Rust](https://www.rust-lang.org/) tutorials gave me rules such as "one mutable reference or many immutable," "moves invalidate the source," and "lifetimes must not outlive their referent." Stack architecture, computability constraints, and Rice's theorem sit underneath these rules and explain every decision Rust's designers made. Dig into them and the rules stop requiring memorization because they become derivable. The borrow checker becomes a consequence of what memory is, what compilers can analyze, and what tradeoffs Rust chose.

I did not find that path in any existing course. [The Rust Book](https://doc.rust-lang.org/book/), [Google's Comprehensive Rust](https://google.github.io/comprehensive-rust/), [r4cppp](https://github.com/nrc/r4cppp), [Codecademy](https://www.codecademy.com/learn/rust-for-programmers), and [Udemy](https://www.udemy.com/courses/search/?src=ukw&q=rust) teach the rules well. This course derives them. SPACE x TIME x COORDINATES is a framework for memory bugs. Use-after-free, data races, and dangling references are failures in one of these three dimensions. The course builds the framework first, and the borrow checker falls out of it.

### Who this course is for

This course is for experienced programmers, particularly those with a C++ background, who already have a mental model of systems programming. The course updates that model. The density is high and the chapters are short, because we believe every sentence should be valuable and add to what you already know.

If you are new to programming or new to systems languages, start with [The Rust Book](https://doc.rust-lang.org/book/). It builds the mental model this course assumes you already have. Come back here when you want to understand the system underneath the rules.

---

## Chapters

<ul class="chapter-list">
  <li>
    <a href="/learn-rust/first-contact/">
      <span class="ch-num">01</span> <span class="ch-title">First Contact</span>
      <span class="ch-desc">Before: Experienced C++ programmer. Has not written Rust.<br>After: Knows Rust catches bugs C++ misses. Does not know how yet. Curious.</span>
    </a>
  </li>
  <li>
    <a href="/learn-rust/space-time-coordinates/">
      <span class="ch-num">02</span> <span class="ch-title">Space, Time, Coordinates</span>
      <span class="ch-desc">Before: Knows Rust catches bugs C++ misses. Does not know how.<br>After: Has a framework for understanding memory bugs. Ready to see how Rust applies these ideas.</span>
    </a>
  </li>
  <li>
    <a href="/learn-rust/how-compilers-track-references/">
      <span class="ch-num">03</span> <span class="ch-title">Catching Coherence at Compile Time</span>
      <span class="ch-desc">Before: Has framework: SPACE, TIME, COORDINATES. Does not know how a compiler could detect these problems.<br>After: Understands how Rust makes compile-time coordinate analysis possible. Ready for lifetime syntax and mechanics.</span>
    </a>
  </li>
  <li>
    <a href="/learn-rust/limits-of-compile-time-analysis/">
      <span class="ch-num">04</span> <span class="ch-title">Limits of Compile-Time Analysis</span>
      <span class="ch-desc">Before: Understands how Rust makes compile-time coordinate analysis possible.<br>After: Understands why the compiler must be conservative and what trade-offs that creates.</span>
    </a>
  </li>
  <li class="wip">
    <span class="ch-num">05</span> <span class="ch-title">...</span>
    <span class="ch-desc">This chapter and subsequent chapters are a work in progress.</span>
  </li>
  <li>
    <a href="/learn-rust/exercises/">
      <span class="ch-num">&nbsp;~&nbsp;</span> <span class="ch-title">Exercises</span>
      <span class="ch-desc">Hands-on exercises organized around the framework, not language features. Starts with the explicit! macro — a companion crate that makes Rust's implicit operations visible.</span>
    </a>
  </li>
</ul>
