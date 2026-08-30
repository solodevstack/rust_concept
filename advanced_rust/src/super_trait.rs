use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct Point{
    x : i32,
    y: i32
}
//Because we’ve specified that OutlinePrint requires the Display trait, we can use the to_string function that is automatically implemented for any type that implements Display. If we tried to use to_string without adding a colon and specifying the Display trait after the trait name, we’d get an error saying that no method named to_string was found for the type &Self in the current scope.
//Then, implementing the OutlinePrint trait on Point will compile successfully, and we can call outline_print on a Point instance to display it within an outline of asterisks.



impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point{
  
}

pub fn printer(){
    let p = Point{x: 30,y: 40};
    // print!(" p {}", outline_print(&p))
      print!(" p {:?}", (&p).outline_print())
     
   
}