struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p2.mixup(p1);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


// use std::cmp::PartialOrd;

// fn largest<T :PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x 
//     }
// }
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("1st p.x = {}", p.x());
//      let p = Point { x: 5.1, y: 10.3 };
//     println!("p.x = {}", p.x);
//     println!("p.y = {}", p.y);
//     println!("Distance from origin = {}", p.distance_from_origin());


    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {result}");
// }




// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['b','y', 'm', 'a', 'q', 'z'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");
// }