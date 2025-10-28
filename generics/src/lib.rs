
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {} same type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {} diffent types
//pub fn notify(item: &(impl Summary + Display)) {} Specifying Multiple Trait Bounds with the + Syntax
//pub fn notify<T: Summary + Display>(item: &T) {} diffent types


// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }
//Clearer Trait Bounds with where Clauses
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     format!("{} - {:?}", t.clone(), u.clone())
// }

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}





pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,

}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
