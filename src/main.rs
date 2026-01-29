mod guessing_game;
use std::io;

fn main() {
    loop {
        println!("\nMain Menu:");
        println!("1. Guessing game");
        println!("2. Others");
        println!("3. Exit");
        println!("Please choose an option:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim() {
            "1" => guessing_game::play(),
            "2" => println!("You chose Option 2: Others (Not implemented yet)"),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
