
use main_file::List::{Cons, Nil};
use main_file::mybox_run;
use main_file::MyBox;
use main_file::CustomSmartPointer;
use std::mem::drop;



//The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to store data on the heap rather than the stack.
//What remains on the stack is the pointer to the heap data.

 //You’ll use them most often in these situations:

//When you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that requires an exact size
//When you have a large amount of data, and you want to transfer ownership but ensure that the data won’t be copied when you do so
//When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type


fn main() {
    let b = Box::new(5);
    // println!("b = {b}");
    //  let list = Cons(1, Cons(2, Cons(3, Nil)));
     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

     let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    mybox_run();

    print_list(&list);
    
     println!("this is b {:?}" ,b);

     let m = MyBox::new(String::from("Rust"));
    hello(&m);

    //Running Code on Cleanup with the Drop Trait
    //We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t call the drop method explicitly. So, if we need to force a value to be cleaned up early, we use the std::mem::drop function.
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
        
    };
    //The std::mem::drop function is different from the drop method in the Drop trait. We call it by passing as an argument the value we want to force-drop. The function is in the prelude, so we can modify main
     drop(c);
    println!("CustomSmartPointer dropped before the end of main");


    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");


    //rc

    //ref_counted::second_main();
    //ref_cell::refcell_main();
    ref_cycles::refcycle_main();

}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn print_list(list: &main_file::List) {
    match list {
        Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        }
        Nil => println!("End"),
    }
}