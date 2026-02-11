//! Exercise 1: Choose Your Coherence Strategy
//!
//! You have shared IDENTITY + mutation. Pick a strategy.

use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};

struct Config {
    timeout_ms: u64,
    max_connections: u64,
}

fn example() {
    // Scenario: Config read by many threads, occasionally updated

    // Option A: RwLock (many readers, occasional writer)
    let config_a = Arc::new(RwLock::new(Config {
        timeout_ms: 1000,
        max_connections: 100
    }));

    println!("RwLock timeout: {}", config_a.read().unwrap().timeout_ms);
    config_a.write().unwrap().timeout_ms = 2000;
    println!("RwLock updated: {}", config_a.read().unwrap().timeout_ms);

    // Option B: Atomics (if config is simple enough)
    let timeout_b = AtomicU64::new(1000);

    println!("Atomic timeout: {}", timeout_b.load(Ordering::Relaxed));
    timeout_b.store(2000, Ordering::Relaxed);
    println!("Atomic updated: {}", timeout_b.load(Ordering::Relaxed));

    // Option C: Arc swap (immutable snapshots)
    // Store Arc<Config>, swap atomically for updates
    // Readers get a snapshot, never block
}

fn exercise() {
    // TODO: Implement Option C (Arc swap) for the Config scenario
    //
    // Hints:
    // - Use arc_swap crate, or implement with AtomicPtr
    // - Readers clone the Arc (cheap)
    // - Writers create new Config, swap the Arc
    //
    // When to use each?
    // - RwLock: complex config, infrequent writes
    // - Atomics: simple values, frequent access
    // - Arc swap: read-heavy, writers create new version
    //
    // Question: What are the tradeoffs of each approach?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
