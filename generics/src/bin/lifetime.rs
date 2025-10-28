// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }   

//no specified lifetime
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// fn main() {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {result}");
//     }
// }
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");
// }
 //return x
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}
// Method independent of the lifetime parameter
// This method doesn't return any references tied to the struct's lifetime,
// so it doesn't depend on the lifetime 'a.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
// Method dependent on the lifetime parameter
// This method returns a reference (self.part) which is tied to the struct's lifetime 'a,
// so it depends on the lifetime to ensure the returned reference is valid.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}


// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//     print!("{first_sentence}");
//     println!("Important excerpt: {}", i.part);
//     let part = i.announce_and_return_part("This is an important announcement!");
//     println!("Part: {}", part);
//     println!("Level: {}", i.level());
// }

//Generic Type Parameters, Trait Bounds, and Lifetimes Together
// This function takes two string slices and an announcement, returning the longer string slice.  
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
fn main(){
    let str1 = String::from("long string is long");
    let str2 = String::from("xyz");
    let result = longest_with_an_announcement(str1.as_str(), str2.as_str(),
        "This is an announcement!");
    println!("The longest string is {result}");
}