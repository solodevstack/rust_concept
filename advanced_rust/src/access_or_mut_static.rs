//Accessing or Modifying a Mutable Static Variable
//In this book, we’ve not yet talked about global variables, which Rust does support but which can be problematic with Rust’s ownership rules. If two threads are accessing the same mutable global variable, it can cause a data race.


//he names of static variables are in SCREAMING_SNAKE_CASE by convention. Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly.
static HELLO_WORLD: &str = "Hello, world!";

fn static_var() {
    println!("value is: {HELLO_WORLD}");
}

//A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used. Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is unsafe. Listing 20-11 shows how to declare, access, and modify a mutable static variable named COUNTER.

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//You must either explicitly opt out of that lint’s protections by adding an #[allow(static_mut_refs)] annotation or access the mutable static variable via a raw pointer created with one of the raw borrow operators. That includes cases where the reference is created invisibly, as when it is used in the println! in this code
#[allow(static_mut_refs)]
pub fn counter() {
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}