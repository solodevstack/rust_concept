use generics::{SocialPost, Summary, NewsArticle,notify};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
        let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    
    // notify(&article)


    println!("New article available! {} ", article.summarize());


    println!("1 new social post: {}", post.summarize());
}
// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// fn main() {
//     let origin = Point { x: 0, y: 0 };


// // assert_eq!(format!("The origin is: {origin}"), "The origin is: (0, 0)");
// print!("The origin is: {origin}");
    
// }