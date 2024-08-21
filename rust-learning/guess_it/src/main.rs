use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("hello welcome to guessing game");

    let seceret_num: i32 = rand::thread_rng().gen_range(1..=100);
    // println!("number gen {seceret_num}");
    loop {
        println!("guess a number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("pls enter a valid input");
                continue;},
        };
        // println!("You guessed: {}", guess_num);
        match guess.cmp(&seceret_num) {
            Ordering::Equal => {
                println!("you won!!!");
                break;
            }
            Ordering::Greater => println!("Too Big try lesser"),
            Ordering::Less => println!("Too small try bigger"),
        }
    }
}
