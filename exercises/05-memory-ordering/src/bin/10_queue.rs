//! Exercise 10: Designing with Orderings
//!
//! Design exercise: simple single-producer single-consumer queue

use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::thread;

const SIZE: usize = 16;

struct SpscQueue<T> {
    buffer: [UnsafeCell<Option<T>>; SIZE],
    head: AtomicUsize,  // Next slot to write (producer)
    tail: AtomicUsize,  // Next slot to read (consumer)
}

unsafe impl<T: Send> Send for SpscQueue<T> {}
unsafe impl<T: Send> Sync for SpscQueue<T> {}

impl SpscQueue<i32> {
    const fn new() -> Self {
        const NONE: UnsafeCell<Option<i32>> = UnsafeCell::new(None);
        SpscQueue {
            buffer: [NONE; SIZE],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }
}

impl<T> SpscQueue<T> {

    fn push(&self, value: T) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let next = (head + 1) % SIZE;

        if next == self.tail.load(Ordering::Acquire) {
            return false;  // Full
        }

        unsafe { *self.buffer[head].get() = Some(value); }
        self.head.store(next, Ordering::Release);  // Publish
        true
    }

    fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);

        if tail == self.head.load(Ordering::Acquire) {
            return None;  // Empty
        }

        let value = unsafe { (*self.buffer[tail].get()).take() };
        self.tail.store((tail + 1) % SIZE, Ordering::Release);
        value
    }
}

fn example() {
    static QUEUE: SpscQueue<i32> = SpscQueue::new();

    let producer = thread::spawn(|| {
        for i in 0..10 {
            while !QUEUE.push(i) {
                std::hint::spin_loop();
            }
        }
    });

    let consumer = thread::spawn(|| {
        for _ in 0..10 {
            loop {
                if let Some(v) = QUEUE.pop() {
                    println!("Got: {}", v);
                    break;
                }
                std::hint::spin_loop();
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

fn exercise() {
    // TODO: Analyze the ordering choices in SpscQueue
    //
    // 1. Why is head.load Relaxed in push but tail.load Acquire?
    // 2. Why is head.store Release?
    // 3. What would break if we used Relaxed everywhere?
    //
    // Map to: Release on write publishes the data
    //         Acquire on read sees the published data

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
