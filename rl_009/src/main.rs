struct User<'a>  {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let user1 = User {
        username: "liushuailong",
        email: "liushuailong@liushuailong.cn",
        sign_in_count: 1,
        active: true,
    };
    println!("Hello, world!");
}
