use aggregator::{Summary, TWeet};
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, ad you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    println!("Hello, world!");
}

