fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    match six {
        None => println!("the value is none!") ,
        Some(i) => println!("the six is {}", i),
    }
    match none {
        None => println!("the value is none!"),
        Some(i) => println!("the six is {}", i),
    }
    println!("Hello, world!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
