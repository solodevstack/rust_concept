//Mutex is an abbreviation for mutual exclusion, as in a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.

//Mutexes have a reputation for being difficult to use because you have to remember two rules:

//You must attempt to acquire the lock before using the data.
//When you’re done with the data that the mutex guards, you must unlock the data so that other threads can acquire the lock.

use std::sync::Mutex;

pub fn single_main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

// use std::thread;

// fn err_mutexmain() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }
// use std::rc::Rc;
// use std::thread;

// fn error_multi_main() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }