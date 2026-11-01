This is a good time to pause and consider how the code in Listings 21-18, 21-19, and 21-20
would be different if we were using futures instead of a closure for the work to be done. What
https://doc.rust-lang.org/book/print.html
679/7196/7/26, 12:02 AM
The Rust Programming Language
types would change? How would the method signatures be different, if at all? What parts of the
code would stay the same?
After learning about the while let loop in Chapter 17 and Chapter 19, you might be
wondering why we didn’t write the Worker thread code as shown in Listing 21-21.
Filename: src/lib.rs
// --snip--
impl Worker {
fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
let thread = thread::spawn(move || {
while let Ok(job) = receiver.lock().unwrap().recv() {
println!("Worker {id} got a job; executing.");
job();
}
});
Worker { id, thread }
}
}
Listing 21-21: An alternative implementation of Worker::new using while let
This code compiles and runs but doesn’t result in the desired threading behavior: A slow
request will still cause other requests to wait to be processed. The reason is somewhat subtle:
The Mutex struct has no public unlock method because the ownership of the lock is based on
the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>> that the lock
method returns. At compile time, the borrow checker can then enforce the rule that a resource
guarded by a Mutex cannot be accessed unless we hold the lock. However, this
implementation can also result in the lock being held longer than intended if we aren’t mindful
of the lifetime of the MutexGuard<T> .
The code in Listing 21-20 that uses let job = receiver.lock().unwrap().recv().unwrap();
works because with let , any temporary values used in the expression on the right-hand side
of the equal sign are immediately dropped when the let statement ends. However, while
let (and if let and match ) does not drop temporary values until the end of the associated
block. In Listing 21-21, the lock remains held for the duration of the call to job() , meaning
other Worker instances cannot receive jobs.