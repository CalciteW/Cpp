#![allow(dead_code)]
use std::fmt;

struct Friend {
    name: String,
    age: u32,
}

impl fmt::Display for Friend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", self.name, self.age)
    }
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
pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let zhangziyao = Friend {
        name: String::from("张子耀"),
        age: 14,
    };

    println!("{}", zhangziyao);

    let tweet = Tweet {
        username: String::from("Yao"),
        content: String::from(" 我是一个阳光开朗的男孩，但我也不是好欺负的"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
