
fn main() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {}", s2);
    println!("Hello, world!");
}
