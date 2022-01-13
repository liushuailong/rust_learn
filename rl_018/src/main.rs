#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let v: Vec<i32> = Vec::new();
    let mut a = vec![1, 2, 3];
    a.push(5);
    a.push(6);
    a.push(7);

    let s = vec![1, 2, 3, 5, 6, 7];
    let third: &i32 = &s[2];
    println!("the third element is {}", third);

    match s.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element."),
    }

    // let does_not_exist = &s[100];
    let does_not_exist = s.get(100);
    match does_not_exist {
        Some(third) => println!("the 100 element is {}", third),
        None => println!("there is no 100 element."),
    }

    for i in &s {
        println!("{}", i);
    }

    let mut m = vec![1, 2, 4, 7];
    for i in &mut m {
        *i += 50;
    }

    for i in &m {
        println!("the add 1: {}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("the row is {:#?}", row);

    
}
