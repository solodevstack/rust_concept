use std::thread;

pub fn move_main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
       
    });
      // drop(v); drop not possible

    handle.join().unwrap();
}