trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

//Running this code will print *waving arms furiously*, showing that Rust called the fly method implemented on Human directly.
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
pub fn non_specific() {
    let person = Human;
    person.fly();


}
pub fn specific(){
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

pub mod animals {
        trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
// You only need to use this more verbose syntax in cases where there are multiple implementations that use the same name and Rust needs help to identify which implementation you want to call.
    pub fn specific_animal() {
        println!("A baby dog is called a {}", Dog::baby_name());

        // cannot call associated function of trait
        //  println!("A baby dog is called a {}", Animal::baby_name());
          println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
          //<Type as Trait>::function(receiver_if_method, next_arg, ...);
    }

}