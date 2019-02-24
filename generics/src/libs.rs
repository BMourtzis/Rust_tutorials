//NOTE: Traits
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more from {}...)", self.summarize_author)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String, 
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item: impl Summary + Display) {
    println!("something");
}

fn some_fn<T,U>(t: T, u: U) -> i32 
    where T: Display + Clone, U: Clone + Debug 
{
    
}

// return impl Summary only works if we return a single type
fn returns_summarize() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("something"),
        reply: false,
        retweet: false
    }
}