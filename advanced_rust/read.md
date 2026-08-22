Advanced rust is a panoply of Rust features with something for everyone! Let’s dive in!

Unsafe Rust

Performing Unsafe Superpowers
Those superpowers include the ability to:

Dereference a raw pointer.
Call an unsafe function or method.
Access or modify a mutable static variable.
Implement an unsafe trait.
Access fields of unions.

Dereference a raw pointer.
Unsafe Rust has two new types called raw pointers that are similar to references. 
 As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively. 
 The asterisk isn’t the dereference operator; it’s part of the type name. 
  In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.
Different from references and smart pointers, raw pointers:

Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
Aren’t guaranteed to point to valid memory
Are allowed to be null
Don’t implement any automatic cleanup
fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;
}
We’ve created raw pointers by using the raw borrow operators: &raw const num creates a *const i32 immutable raw pointer, and &raw mut num creates a *mut i32 mutable raw pointer.
With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race. Be careful!

With all of these dangers, why would you ever use raw pointers? One major use case is when interfacing with C code, as you’ll see in the next section. Another case is when building up safe abstractions that the borrow checker doesn’t understand. We’ll introduce unsafe functions and then look at an example of a safe abstraction that uses unsafe code.

   use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
Listing 20-7: Creating a slice from an arbitrary memory location
We don’t own the memory at this arbitrary location, and there is no guarantee that the slice this code creates contains valid i32 values. Attempting to use values as though it’s a valid slice results in undefined behavior

Using extern Functions to Call External Code

Within the unsafe extern "C" block, we list the names and signatures of external functions from another language we want to call. The "C" part defines which application binary interface (ABI) the external function uses: The ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the C programming language’s ABI. 

Implementing an Unsafe Trait

We can use unsafe to implement an unsafe trait. A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify. We declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too, as shown in Listing 20-12.

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
As an example, recall the Send and Sync marker traits we discussed in the “Extensible Concurrency with Send and Sync” section in Chapter 16: The compiler implements these traits automatically if our types are composed entirely of other types that implement Send and Sync. If we implement a type that contains a type that does not implement Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe. Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with unsafe.

Accessing Fields of a Union
The final action that works only with unsafe is accessing fields of a union. A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance. https://doc.rust-lang.org/reference/items/unions.html

Using Miri to Check Unsafe Code

When writing unsafe code, you might want to check that what you have written actually is safe and correct. One of the best ways to do that is to use Miri, an official Rust tool for detecting undefined behavior. Whereas the borrow checker is a static tool that works at compile time, Miri is a dynamic tool that works at runtime. It checks your code by running your program, or its test suite, and detecting when you violate the rules it understands about how Rust should work. Using Miri requires a nightly build of Rust.
 You can install both a nightly version of Rust and the Miri tool by typing rustup +nightly component add miri. 
  cargo +nightly miri run
  Miri doesn’t catch everything you might get wrong when writing unsafe code. Miri is a dynamic analysis tool, so it only catches problems with code that actually gets run. That means you will need to use it in conjunction with good testing techniques to increase your confidence about the unsafe code you have written. Miri also does not cover every possible way your code can be unsound.