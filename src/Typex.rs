struct Point<T>{
    x:T,
    y:T,
}

impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}

struct Pointx<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pointx<T, U> {
    fn mixup<V, W>(self, other: Pointx<V, W>) -> Pointx<T, W> {
        Pointx {
            x: self.x,
            y: other.y,
        }
    }
}



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

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
