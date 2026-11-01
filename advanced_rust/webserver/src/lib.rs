//If you’re feeling ambitious, try to write a function named build with the
//following signature to compare with the new function:
//pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {

// Note: If the operating system can’t create a thread because there aren’t enough system
// resources, thread::spawn will panic. That will cause our whole server to panic, even
// https://doc.rust-lang.org/book/print.html
// 671/7196/7/26, 12:02 AM
// The Rust Programming Language
// though the creation of some threads might succeed. For simplicity’s sake, this behavior is
// fine, but in a production thread pool implementation, you’d likely want to use
// std::thread::Builder and its spawn method that returns Result instead.

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// In this case, a better alternative exists: the Vec::drain method. It accepts a range parameter
// to specify which items to remove from the vector and returns an iterator of those items.
// Passing the .. range syntax will remove every value from the vector.

// This resolves the compiler error and does not require any other changes to our code. Note
// that, because drop can be called when panicking, the unwrap could also panic and cause a
// double panic, which immediately crashes the program and ends any cleanup in progress. This
// is fine for an example program, but it isn’t recommended for production code.

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}
impl ThreadPool {
    /// Create a new ThreadPool
    /// The `new` function.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            // let id = size + 1;
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        //    Worker { id, thread: thread::spawn(|| {
        //     reciever;
        //    }) }
        // let thread = thread::spawn(|| {
        //     receiver;
        // });
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");
                job();
            }
        });

        Worker { id, thread }
    }
}
