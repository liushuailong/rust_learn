fn main() {
    println!("5 feibonaqi is {}", fobonaqi(5));
    println!("6 feibonaqi is {}", fobonaqi(6));
    println!("Hello, world!");
}

fn fobonaqi(n: i32) -> i64 {
    if n == 1 {
        return 1 as i64;
    }
    if n == 2 {
        return 1 as i64;
    }
    fobonaqi(n - 1) + fobonaqi(n -2)
}
