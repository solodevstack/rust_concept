use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
//When you see code that uses await, Rust compiles it under the hood to code that calls poll. If you look back at Listing 17-4, where we printed out the page title for a single URL once it resolved, Rust compiles it into something kind of (although not exactly) like this:
// match page_title(url).poll() {
//     Ready(page_title) => match page_title {
//         Some(title) => println!("The title for {url} was {title}"),
//         None => println!("{url} had no title"),
//     }
//     Pending => {
//         // But what goes here?
//     }
// }

//What should we do when the future is still Pending? We need some way to try again, and again, and again, until the future is finally ready. In other words, we need a loop:

// let mut page_title_fut = page_title(url);
// loop {
//     match page_title_fut.poll() {
//         Ready(value) => match page_title {
//             Some(title) => println!("The title for {url} was {title}"),
//             None => println!("{url} had no title"),
//         }
//         Pending => {
//             // continue
//         }
//     }
// }