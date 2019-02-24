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

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrb> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y 
        {
            println!("The largest member is x = {}", x);
        }
        else
        {
            println!("The largest member is y = {}", y);
        }
    }
}

impl<T: Display> ToString for T {
    //
}

//Lifetimes
fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz");

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str 
{
    if x.len() > y.len() 
    {
        x
    }
    else
    {
        y
    }
}

struct ImportantExcerpt<'a> 
{
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt{ part: first_sentence };
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len()
    {
        x
    }
    else
    {
        y
    }
}