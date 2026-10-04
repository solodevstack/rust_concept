// The term macro refers to a family of features in Rust—declarative macros with macro_rules! and three kinds of procedural macros:

// Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// Attribute-like macros that define custom attributes usable on any item
// Function-like macros that look like function calls but operate on the tokens specified as their argument
//Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.

use advanced_rust::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

pub fn macro_main() {
    Pancakes::hello_macro();
}