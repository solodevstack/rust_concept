
use smart_pointers::List::{Cons, Nil};

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
    print_list(&list);
    
     println!("{:?}" ,b)

}
fn print_list(list: &smart_pointers::List) {
    match list {
        Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        }
        Nil => println!("End"),
    }
}