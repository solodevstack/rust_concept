//Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. 
//One example of a trait with an associated type is the Iterator trait that the standard library provides. The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run_counter(){
    let counts = vec![1,2,3,4,];

    let mut counter = Counter::new();
    for _ in counts{
        counter.next();
    }
    print!("{:?}",counter)


}