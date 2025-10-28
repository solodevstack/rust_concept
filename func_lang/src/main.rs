
#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // These are all valid definitions that will produce the same behavior when they’re called
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;
    // The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked into the closure in example_closure, and we get a type error when we next try to use a different type with the same closure.
    // let example_closure = |x| x;
    //  let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    //This example also illustrates that a variable can bind to a closure definition, and we can later call the closure by using the variable name and parentheses as if the variable name were a function name.
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let only_borrows = || println!("From closure: {list:?}");

    // println!("Before calling closure: {list:?}");
    // only_borrows();
    // println!("After calling closure: {list:?}");
    let mut list = vec![1, 2, 3];
        println!("1st Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(7);
        //Between the closure definition and the closure call, an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow.
        
        borrows_mutably();
        println!("After calling closure: {list:?}");

  use std::thread;


    let list = vec![1, 2, 3];
    println!("thread: Before defining closure: {list:?}");

    //The new thread might finish before the rest of the main thread finishes, or the main thread might finish first. If the main thread maintained ownership of list but ended before the new thread did and dropped list, the immutable reference in the thread would be invalid. Therefore, the compiler requires that list be moved into the closure given to the new thread so the reference will be valid.

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
//move moves ownership of s into the closure, yes.

// But: inside the closure, you are only borrowing (&s) immutably when you call println!.
// You never actually move s out or mutate it inside the closure.
//       let s = String::from("Hello");

//     // `move` forces the closure to take ownership of `s`
//     let consume = move || {
//         println!("Consumed: {}", s);
//         // `s` is moved into the closure, so it’s gone after this call
//     };
//     consume(); // ✅ works
//You need to consume (move) the captured variable inside the closure. Example:

//     let s = String::from("Hello");

// let consume = move || {
//     println!("Consumed: {}", s);
// };

//     consume(); // ✅ works
//     consume(); // ✅ works

sort();
iterator_demonstration();
}
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

#[derive(Debug)]
   struct Rectangle {
    width: u32,
    height: u32,
}

fn sort() {
 
    //  let mut list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // list.sort_by_key(|r| r.width);
    // println!("{list:#?}");
    //This closure can be called once; trying to call it a second time wouldn’t work because value would no longer be in the environment to be pushed into sort_operations again! Therefore, this closure only implements FnOnce. When we try to compile this code, we get this error that value can’t be moved out of the closure because the closure must implement FnMut
    // let mut list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

    #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

 
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.height
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}


// Let’s look at the definition of the unwrap_or_else method on Option<T> that we used in Listing 13-1:


// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }
