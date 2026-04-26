pub enum List {
    Cons(i32, Box<List>),
    Nil,
}


pub struct MyBox<T>(T);

impl<T> MyBox<T> {
   pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub fn mybox_run () {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
use std::ops::Deref;


pub struct CustomSmartPointer {
   pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
