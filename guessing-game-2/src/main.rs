use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("🆕 Let's start the guessing game!!\n");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    loop {
        guess.clear();

        println!("Guess a number between 1 and 100");

        io::stdin()
            .read_line(&mut guess)
            .expect("An error occured while readiing the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Errored out!");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("The entered value is EQUAL!!!!");
                break;
            }
            Ordering::Greater => println!("The entered value is GREATER than expected"),
            Ordering::Less => println!("The entered value is LESSER than the secret number"),
        }
    }
}
