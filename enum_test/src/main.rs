//option<T> is a powerful feature in Rust that allows you to represent a value that may or may not be present.

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = match y {
    Some(val) => x + val,
    None => x, // or handle it differently
};

println!("Sum is {}", sum);
}
//Matching with Option<T>

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);



//test
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {
//     let result = add(2, 6);
//     assert_eq!(result, 5);
// }








//Enums methods

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//     impl Message {
//         fn call(&self) {
//             match self {
//             Message::Quit => println!("The Quit variant was called."),
//             Message::Move { x, y } => {
//                 println!("Move to coordinates: ({}, {})", x, y);
//             }
//             Message::Write(text) => {
//                 println!("Write message: {}", text);
//             }
//             Message::ChangeColor(r, g, b) => {
//                 println!("Change color to RGB({}, {}, {})", r, g, b);
//             }
//             }
          
//         }
//     }


// fn main() {
    
//     let m = Message::Move { x: 10, y: 20 };
//     m.call();

   
// }


// This example demonstrates how to use enums in Rust to represent different types of IP addresses.// It shows two approaches: using an enum with a separate struct for the address,
// and using an enum with data directly associated with its variants.
// Enums are a powerful feature in Rust that allow you to define a type that can be
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

// Using an enum with data
// This is a more concise way to define the same functionality using an enum with data.
// It allows you to store the address directly in the enum variants.
// This approach is often preferred for its simplicity and clarity.
// This example demonstrates how to use an enum with data to represent IP addresses.
// It eliminates the need for a separate struct and allows you to directly associate data with each enum variant.
    //     enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));

// This example demonstrates how to use enums in Rust to represent different types of messages.
// It shows how to define an enum with different variants, including a unit variant, a struct variant, and a tuple variant.
// Enums are useful for defining a type that can take on different
