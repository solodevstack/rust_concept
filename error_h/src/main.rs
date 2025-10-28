use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    match read_username_from_file2() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading file: {}", e),
    }
    // read_username_from_file3();
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
  
}
fn read_username_from_file3() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
  println!("Username: {}", greeting_file.metadata().unwrap().len());
}

//cleaner
//use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {error:?}");
//             })
//         } else {
//             panic!("Problem opening the file: {error:?}");
//         }
//     });
// }
//Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.


// fn panic() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {error:?}"),
//     };
// }
