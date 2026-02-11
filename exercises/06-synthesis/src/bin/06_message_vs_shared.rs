//! Exercise 6: Message Passing vs Shared State
//!
//! Same problem, two approaches

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;

fn message_passing() {
    println!("=== Message Passing ===");
    let (tx, rx) = mpsc::channel();

    // Producers
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("Message from producer {}", i)).unwrap();
        });
    }
    drop(tx);  // Close sender so receiver knows when done

    // Consumer
    for msg in rx {
        println!("Received: {}", msg);
    }
}

fn shared_state() {
    println!("\n=== Shared State ===");
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    // Producers
    let mut handles = vec![];
    for i in 0..3 {
        let queue = queue.clone();
        handles.push(thread::spawn(move || {
            queue.lock().unwrap().push_back(format!("Message from producer {}", i));
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Consumer
    while let Some(msg) = queue.lock().unwrap().pop_front() {
        println!("Received: {}", msg);
    }
}

fn example() {
    message_passing();
    shared_state();

    // Channels: no coherence problem (no shared SPACE)
    // Shared state: coherence via TIME serialization (Mutex)
}

fn exercise() {
    // TODO: Implement a producer-consumer with bounded queue
    //
    // Requirements:
    // - Fixed capacity (e.g., 5 items)
    // - Producers block when full
    // - Consumers block when empty
    //
    // Try both approaches:
    // 1. Using channels (mpsc::sync_channel)
    // 2. Using shared state (Mutex + Condvar)
    //
    // Question: Which is simpler? Which is more flexible?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
