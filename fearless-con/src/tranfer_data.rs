//“Do not communicate by sharing memory; instead, share memory by communicating.”
//A channel is a general programming concept by which data is sent from one thread to another.
//A channel is said to be closed if either the transmitter or receiver half is dropped.

use std::sync::mpsc;
//mpsc stands for multiple producer, single consumer. 
use std::thread;
use std::time::Duration;

pub fn tf_main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    //We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel. 
    //We could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.
    let rc = rx.recv().unwrap();
    println!(" result {}", rc);
}

// fn err_main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         println!("val is {val}");
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {received}");
// }

pub fn multi_main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
pub fn sec_multi(){
     let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
     thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
  

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

}