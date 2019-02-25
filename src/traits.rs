use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_a(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notyfy<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
}

pub fn notify_a<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function_a<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    7
}

fn some_function_b<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
    7
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

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use crate::traits::Tweet;
    use crate::traits::Summary;
    use crate::traits::notify;
    use crate::traits::largest;

    #[test]
    fn a() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize_a());
        println!("1 new tweet: {}", tweet.summarize());

        notify(tweet);
    }

    #[test]
    fn b() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
}