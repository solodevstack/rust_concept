pub fn deref_raw() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    //shows how to create a raw pointer to an arbitrary location in memory. Trying to use arbitrary memory is undefined: There might be data at that address or there might not, the compiler might optimize the code so that there is no memory access, or the program might terminate with a segmentation fault. 
    // let address = 0x012345usize;
    // let r = address as *const i32;
     unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    
  
}