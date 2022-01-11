use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    // create a secret number by rand
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("the secret number is {}", secret_number);
    // create var to receive the value for io::stdin.
    loop {
        println!("pls input your guess.");
        let mut guess = String::new();
        // let user to input the content at termianl console;
        // and use the variable guess to save the value.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        println!("you guessed: {}", guess);
        // transform the String to number
        // let guess: u32 = guess.trim().parse().expect("pls type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // compare the secret num to the input num
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too samll!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
