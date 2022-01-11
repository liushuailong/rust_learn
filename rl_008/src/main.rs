fn main() {
    let s = String::from("string envum");
    println!("the word is {}", first_word(&s));
    println!("Hello, world!");
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
