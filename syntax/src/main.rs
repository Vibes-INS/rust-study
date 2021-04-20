use syntax::Tweet;
use syntax::Summary;
use syntax::notify;

fn main() {
    println!("Hello, world!");
    let number_list = [ 1, 2, 3, 4, 5 ];
    let x = largest(&number_list);
    println!("{}", x);

    part1();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item
        }
    }
    max
}

fn part1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    notify(tweet);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}