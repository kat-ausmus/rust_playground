use std::io;

pub fn play() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Variable Ownership Exercises!");
    println!("Please input a word or a sentence:");
    let mut s1 = String::new();
    io::stdin()
        .read_line(&mut s1)
        .expect("Failed to read line.");
    let mut trimmer_str = s1.trim_end().to_string();
    if trimmer_str.is_empty() {
        trimmer_str = String::from("a quick brown fox jump over the lazy cow");
    }

    let len = calculate_length(&trimmer_str);

    println!("** Calculated Length");
    println!("The length of '{trimmer_str}' is {len}.\n");

    let num_words = calculate_number_of_words(&trimmer_str);

    println!("The string '{trimmer_str}' has {num_words} words.\n");

    println!("Please input which word position you want to return:\n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let word_position : u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number, please try again.");
            return;
        }
    };
    let word_position_name = match word_position {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        _ => format!("{}th", word_position)
    };
    println!("The {word_position_name} word is: {}", return_the_nth_word(&trimmer_str, num_words, word_position as usize));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_number_of_words(s: &String) -> usize {
    s.split_whitespace().count()
}

fn return_the_nth_word(s: &String, num_words: usize, position: usize) -> String {
    if position > num_words {
        return "".to_string();
    }
    let real_position = position - 1;
    s.split_whitespace().nth(real_position).unwrap_or_default().to_string()
}
