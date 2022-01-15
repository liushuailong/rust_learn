use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 60);
    match scores.get("Blue") {
        Some(val) =>  println!(" the value of blue is {}", val),
        None => println!("the scores not has the key named blue"),
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("the scores is {:#?}", scores);
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }
    scores.insert(String::from("Blue"), 20);
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }
    scores.entry(String::from("red")).or_insert(70);
    scores.entry(String::from("Blue")).or_insert(70);
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }
    
    let text = "hello world worderful world"; 
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("the map is {:#?}", map);

    println!("Hello, world!");
}
