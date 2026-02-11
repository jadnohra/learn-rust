//! Exercise 7: Double-Checked Locking
//!
//! A classic pattern that requires careful ordering

use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Mutex;
use std::ptr;
use std::thread;

static INSTANCE: AtomicPtr<String> = AtomicPtr::new(ptr::null_mut());
static INIT_LOCK: Mutex<()> = Mutex::new(());

fn get_instance() -> &'static String {
    let mut ptr = INSTANCE.load(Ordering::Acquire);  // See initialized data

    if ptr.is_null() {
        let _lock = INIT_LOCK.lock().unwrap();
        ptr = INSTANCE.load(Ordering::Relaxed);  // Recheck under lock

        if ptr.is_null() {
            let s = Box::new(String::from("initialized"));
            ptr = Box::into_raw(s);
            INSTANCE.store(ptr, Ordering::Release);  // Publish
        }
    }

    unsafe { &*ptr }
}

fn example() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            println!("{:?}: {}", thread::current().id(), get_instance());
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    // Release publishes initialized data, Acquire sees it
}

fn exercise() {
    // TODO: What would happen with Relaxed instead of Release on store?
    // TODO: What would happen with Relaxed instead of Acquire on load?
    //
    // Think about: another thread might see the pointer but not the
    // initialized String data. This is a classic double-checked locking bug.
    //
    // Question: Why is the second load (under lock) Relaxed?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
