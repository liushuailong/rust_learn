use std::collections::HashMap;
fn main() {
    let int_vec = vec![1, 3, 5, 8, 2, 4];
    let mut total: i32 = 0;
    let len_vec: i32  = int_vec.len().try_into().unwrap();
    for val in int_vec.into_iter() {
        total += val;
    }
    let mean  = total as f64  / len_vec as f64;
    println!("the mean of value: {}", mean);
    // sort the vector
    let mut int_vec = vec![1, 3, 5, 8, 2, 4, 8];
    int_vec.sort();
    let len_vec = int_vec.len();
    let median: f64;
    if len_vec as i32 % 2 == 0 {
        median = (int_vec[(len_vec ) / 2 - 1] + int_vec[(len_vec) / 2]) as f64 / 2.0;
    } else {
        median = int_vec[(len_vec + 1) / 2 - 1] as f64;
        println!("odd num");
    }
    println!("the len is {}", len_vec);
    println!("{:?}", int_vec);
    println!("the median is {}", median);

    let mut vec_wor = HashMap::new();
    for val in int_vec.into_iter() {
        let count = vec_wor.entry(val).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", vec_wor);
    
    
    println!("Hello, world!");
}
