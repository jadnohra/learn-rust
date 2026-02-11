//! Exercise 3: Thread Pool: IDENTITY Transfer
//!
//! Work items must be Send + 'static

use std::sync::mpsc;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));

        for id in 0..size {
            let receiver = receiver.clone();
            thread::spawn(move || {
                loop {
                    let job = receiver.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            println!("Worker {} executing job", id);
                            job();
                        }
                        Err(_) => {
                            println!("Worker {} shutting down", id);
                            break;
                        }
                    }
                }
            });
        }

        ThreadPool { sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

fn example() {
    let pool = ThreadPool::new(4);

    for i in 0..10 {
        pool.execute(move || {
            println!("Job {} running on {:?}", i, thread::current().id());
            thread::sleep(std::time::Duration::from_millis(100));
        });
    }

    thread::sleep(std::time::Duration::from_secs(2));

    // Jobs must be Send (IDENTITY can cross thread boundary)
    // Jobs must be 'static (no borrowed IDENTITY that might expire)
}

fn exercise() {
    // TODO: Add graceful shutdown to ThreadPool
    //
    // Hints:
    // - Store JoinHandles in the pool
    // - Send a shutdown signal (e.g., Option<Job> where None = shutdown)
    // - impl Drop for ThreadPool that joins all workers
    //
    // Question: Why must jobs be 'static? What would break with borrowed data?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}
