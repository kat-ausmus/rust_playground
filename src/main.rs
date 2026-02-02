mod services;

use std::io;

fn main() {
    loop {
        println!("\nMain Menu:");
        println!("1. Guessing game");
        println!("2. Ownership");
        println!("3. Gigasecond");
        println!("4. Exit");
        println!("Please choose an option:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim() {
            "1" => services::guessing_game::play(),
            "2" => services::ownership_exercise::play(),
            "3" => services::gigasecond::play(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
