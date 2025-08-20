# Trait
    Traits are similar to a feature often called interfaces in other languages
    We can use traits to define shared behavior in an abstract way
    We can use trait bounds to specify that a generic type can be any type that has certain behavior


Simple
```rust
fn main() 
{
    let post = SocialPost 
    {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}

pub trait Summary 
{
    fn summarize(&self) -> String;
}

pub struct NewsArticle 
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle 
{
    fn summarize(&self) -> String 
    {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost 
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost 
{
    fn summarize(&self) -> String 
    {
        format!("{}: {}", self.username, self.content)
    }
}
```

Default Implementations
```rust
fn main() 
{
    let post = SocialPost 
    {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}

pub trait Summary 
{
    fn summarize(&self) -> String 
    {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle 
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle 
{
    fn summarize(&self) -> String 
    {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost 
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {}
```

Force implement
```rust
fn main() 
{
    let post = SocialPost 
    {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}

pub trait Summary 
{
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String 
    {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct SocialPost 
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost 
{
    fn summarize_author(&self) -> String 
    {
        format!("@{}", self.username)
    }
}
```

Parameters
```rust
fn main() 
{
    let post = SocialPost 
    {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    notify(&post);
}

pub trait Summary 
{
    fn summarize(&self) -> String;
}

pub struct NewsArticle 
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle 
{
    fn summarize(&self) -> String 
    {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost 
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost 
{
    fn summarize(&self) -> String 
    {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) 
{
    println!("Breaking news! {}", item.summarize());
}
```

Bound Syntax
```rust
fn main() 
{
    let post = SocialPost 
    {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    notify(&post);
}

pub trait Summary 
{
    fn summarize(&self) -> String;
}

pub struct NewsArticle 
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle 
{
    fn summarize(&self) -> String 
    {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost 
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost 
{
    fn summarize(&self) -> String 
    {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) 
{
    println!("Breaking news! {}", item.summarize());
}
```

If we want this function to allow item1 and item2 to have different types
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

item1 and item2 must be the same type
```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

Specifying Multiple Trait Bounds with the + Syntax
```rust
pub fn notify(item: &(impl Summary + Display)) {
pub fn notify<T: Summary + Display>(item: &T) {
```

Clearer Trait Bounds with where Clauses
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}
```

Returning Types That Implement Traits
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
```

Using Trait Bounds to Conditionally Implement Methods
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

blanket implementations
```rust
impl<T: Display> ToString for T 
{
    // --snip--
}
```
Drop
```rust
fn main() 
{
    {
        println!("Creating st1");
        let _st = ST;
        drop(_st);
    }

    println!("Creating st");
    let _st = ST;

    println!("Finish Main");
}

struct  ST;

impl Drop for ST 
{
    fn drop(&mut self) 
    {
        println!("Dropping st");
    }
}
```