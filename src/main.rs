use std::fmt;

struct Friend {
    name: String,
    age: u32,
}

impl fmt::Display for Friend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.age)
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
        format!(
            "{}, by {} ({})\n{}",
            self.headline, self.author, self.location, self.content
        )
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
        content: String::from("我是一个阳光开朗的男孩，但我也不是好欺负的"),
        reply: false,
        retweet: false,
    };

    let speech_of_yao = NewsArticle {
        headline: String::from("自我介绍"),
        location: String::from("桃城中学"),
        author: String::from("张子耀"),
        content: String::from("我叫张子耀， 来自北京，是一个乐观开朗的男孩，但我也不是好欺负的。"),
    };

    println!("{}", speech_of_yao.summarize());

    println!("{}。", tweet.summarize());
}
