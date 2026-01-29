use std::io;
use std::cmp::Ordering;

pub fn play() {
    println!("Guess the number!");
    let secret_number = rand::random_range(1..=100);
    let mut num_guesses = 0;

    loop {
        num_guesses += 1;
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("is too small!"),
            Ordering::Greater => println!("is too big!"),
            Ordering::Equal => {
                println!("You win after {num_guesses} guesses!");
                break;
            },
        }
    }

    println!("The secret number was: {secret_number}");
}
