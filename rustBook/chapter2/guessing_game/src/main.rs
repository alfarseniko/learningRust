extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //Let's build a guessing game
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
