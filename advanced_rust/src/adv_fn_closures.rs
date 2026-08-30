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

pub fn advanced_fn() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}