//Advanced Functions and Closures
//This section explores some advanced features related to functions and closures, including function pointers and returning closures.

//Function Pointers

//We’ve talked about how to pass closures to functions; you can also pass regular functions to functions! 
// Functions coerce to the type fn (with a lowercase f), not to be confused with the Fn closure trait. The fn type is called a function pointer.

//Passing functions with function pointers will allow you to use functions as arguments to other functions.


fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
//Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.
pub fn advanced_fn() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}

//That said, one example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

pub fn example1(){
        let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();


}

pub fn example2(){
      let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
        print!("status {:?}", list_of_strings);



}
pub fn example3(){
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    print!("status {:?}", list_of_statuses);




}

//Returning Closures
//Closures are represented by traits, which means you can’t return closures directly. 
pub fn example4() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("out {output}");
    }
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32>{
      Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) ->Box<dyn Fn(i32) -> i32> {
     Box::new(move |x| x + init)
}
