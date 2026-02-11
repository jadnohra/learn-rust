//! Exercise 7: The Observer Pattern
//!
//! Multiple observers react to changes

use std::rc::{Rc, Weak};

trait Observer {
    fn on_change(&self, value: i32);
}

struct Subject {
    value: i32,
    observers: Vec<Weak<dyn Observer>>,
}

impl Subject {
    fn new(value: i32) -> Self {
        Subject { value, observers: Vec::new() }
    }

    fn subscribe(&mut self, observer: &Rc<dyn Observer>) {
        self.observers.push(Rc::downgrade(observer));
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;

        // Notify and clean up dead observers
        self.observers.retain(|weak| {
            if let Some(observer) = weak.upgrade() {
                observer.on_change(value);
                true
            } else {
                false  // Observer was dropped
            }
        });
    }
}

struct PrintObserver { name: String }

impl Observer for PrintObserver {
    fn on_change(&self, value: i32) {
        println!("{} sees value: {}", self.name, value);
    }
}

fn example() {
    let mut subject = Subject::new(0);

    let obs1: Rc<dyn Observer> = Rc::new(PrintObserver {
        name: String::from("Observer 1")
    });
    let obs2: Rc<dyn Observer> = Rc::new(PrintObserver {
        name: String::from("Observer 2")
    });

    subject.subscribe(&obs1);
    subject.subscribe(&obs2);

    subject.set_value(10);

    drop(obs1);
    println!("\nDropped Observer 1...\n");

    subject.set_value(20);

    // Weak allows IDENTITY without extending SPACE lifetime
}

fn exercise() {
    // TODO: Make the observer pattern thread-safe
    //
    // Changes needed:
    // - Arc instead of Rc
    // - Arc<Mutex<Vec<...>>> for observers list
    // - Or use RwLock for better read performance
    //
    // Question: What happens if an observer's on_change
    // calls set_value? How do you prevent deadlock?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
