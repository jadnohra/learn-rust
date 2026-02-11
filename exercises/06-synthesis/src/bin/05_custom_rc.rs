//! Exercise 5: Custom Smart Pointer
//!
//! Implement a simple reference-counted pointer

use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;

struct MyRc<T> {
    ptr: NonNull<Inner<T>>,
}

struct Inner<T> {
    value: T,
    count: Cell<usize>,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        let inner = Box::new(Inner {
            value,
            count: Cell::new(1),
        });
        MyRc {
            ptr: NonNull::new(Box::into_raw(inner)).unwrap(),
        }
    }

    fn count(&self) -> usize {
        unsafe { self.ptr.as_ref().count.get() }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let inner = self.ptr.as_ref();
            inner.count.set(inner.count.get() + 1);
        }
        MyRc { ptr: self.ptr }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.ptr.as_ref();
            let count = inner.count.get();
            if count == 1 {
                // Last reference, deallocate
                drop(Box::from_raw(self.ptr.as_ptr()));
            } else {
                inner.count.set(count - 1);
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &self.ptr.as_ref().value }
    }
}

fn example() {
    let a = MyRc::new(42);
    println!("Created a, count: {}", a.count());

    let b = a.clone();
    println!("Cloned to b, count: {}", a.count());

    println!("a = {}, b = {}", *a, *b);

    drop(b);
    println!("Dropped b, count: {}", a.count());

    // When a drops, count reaches 0, Inner deallocated
}

fn exercise() {
    // TODO: Add a weak_count and implement MyWeak<T>
    //
    // struct Inner<T> {
    //     value: T,
    //     strong_count: Cell<usize>,
    //     weak_count: Cell<usize>,
    // }
    //
    // MyWeak should:
    // - Not prevent deallocation of value
    // - Return Option<MyRc<T>> on upgrade()
    // - Only deallocate Inner when both counts are 0
    //
    // Question: Why do we need separate strong and weak counts?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
