



fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[..]);
    // let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    // let word = first_word(&my_string);

    // let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    // let word = first_word(&my_string_literal[0..6]);
    // let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // let word = first_word(my_string_literal);
    println!("The first word is at index: {}", word);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}




// fn main() {
//     let s = String::from("hello world");
//     let word = first_word(&s);
//     println!("The first word is at index: {}", word);
// }
//Bug prone

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         println!("Checking byte: {}", i);
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

