//! Exercise 8: Lock-Free Stack
//!
//! Warning: This is simplified and has the ABA problem!

use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

struct Stack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { head: AtomicPtr::new(ptr::null_mut()) }
    }

    fn push(&self, value: T) {
        let node = Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Relaxed);
            unsafe { (*node).next = head; }

            if self.head.compare_exchange(
                head, node,
                Ordering::Release,
                Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
    }

    fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };

            if self.head.compare_exchange(
                head, next,
                Ordering::Release,
                Ordering::Relaxed
            ).is_ok() {
                let value = unsafe { Box::from_raw(head).value };
                return Some(value);
            }
        }
    }
}

unsafe impl<T: Send> Send for Stack<T> {}
unsafe impl<T: Send> Sync for Stack<T> {}

fn example() {
    let stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());

    // Note: This has the ABA problem!
}

fn exercise() {
    // TODO: Research and explain the ABA problem
    //
    // The ABA problem:
    // 1. Thread 1 reads head = A
    // 2. Thread 2 pops A, pops B, pushes A back
    // 3. Thread 1's CAS succeeds (head is still A)
    // 4. But A.next is now wrong!
    //
    // Solutions:
    // - Hazard pointers
    // - Epoch-based reclamation (crossbeam)
    // - Tagged pointers (version counter)
    //
    // Question: Why doesn't this bug show up in single-threaded use?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
