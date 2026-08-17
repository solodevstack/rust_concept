mod gui_lib;
mod blog_oopdesign;
mod rust_design;

//When we wrote the library, we didn’t know that someone might add the SelectBox type, but our Screen implementation was able to operate on the new type and draw it because SelectBox implements the Draw trait, which means it implements the draw method.

 struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl gui_lib::Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// fn main() {
//     let screen = gui_lib::Screen {
//         components: vec![
//             Box::new( SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No"),
//                 ],
//             }),
//             Box::new(gui_lib::Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from("OK"),
//             }),
//         ],
//     };

//     screen.run();
// }
// fn main() {
//     let mut post = blog_oopdesign::Post::new();

//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());
//     // post.reject();
//     // assert_eq!("", post.content());

//     post.approve();
//     // assert_eq!("I ate a salad for lunch today 2", post.content());
//         post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());
  

// }

    use rust_design::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
