use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

fn main() {
    println!("trait");
    println!("like interface, we can use traits to define shared behavior in an abstract way.");

    let post = SocialPost {
        username: "username".to_string(),
        content: "content".to_string(),
        reply: true,
        repost: true,
    };
    use crate::Summary; // must use trait
    println!("{}", post.summarize());
    println!("{}", post.read());
    println!("{}", notify(&post));
}

// trait
pub trait Summary {
    fn summarize(&self) -> String;
    // can provide default method
    fn read(&self) -> String {
        format!("default - {}", self.summarize())
    }
}

// impl trait(orphan rule/coherence: either trait or type should be local to crate)
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("[summary] {}: {}", self.username, self.content)
    }
}

// trait bound can be used in function parameter and generic type method
// trait bound: same as <T: Summary + Debug>
fn notify(item: &(impl Summary + Debug)) -> String {
    format!("Breaking news: {} [{:?}]", item.summarize(), item)
}
// trait bound: where keyword
fn some_function<T, U>(t: &T, u: &U) -> impl Display
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0 // only same return type here, only to simplified and restrict return type
}

// blanket implementations: impl trait for all types that implement some trait
impl<T: Display> Summary for T {
    fn summarize(&self) -> String {
        format!("[summary] {}", self.to_string())
    }
}
