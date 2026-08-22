
//We must call the dangerous function within a separate unsafe block. If we try to call dangerous without the unsafe block, we’ll get an error:

pub fn unsafe_fn(){
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

}