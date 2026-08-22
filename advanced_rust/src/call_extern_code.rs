


unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn extern_code() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


//Calling Rust Functions from Other Languages
//We can also use extern to create an interface that allows other languages to call Rust functions. Instead of creating a whole extern block,
#[unsafe(no_mangle)]
//annotation to tell the Rust compiler not to mangle the name of this function. Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. 
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
