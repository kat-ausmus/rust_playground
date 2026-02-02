use time::{PrimitiveDateTime as DateTime, Duration, macros::format_description};

// const KILOSECOND: u64 = 1_000;
// const MEGASECOND: u64 = 1_000_000;
const GIGASECOND: i64 = 1_000_000_000;

pub fn play() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // let brownie_cook_time = 2 * KILOSECOND;
    // let vacation_duration = 2 * MEGASECOND;
    // let anniversary = 1 * GIGASECOND;
    // Sample input for now:  Simulate user input (or use std::io::stdin to get it live)
    let input_date = "2023-01-01 12:00:00";

    let description = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    // 3. Parse the string into a PrimitiveDateTime
    let start_date = DateTime::parse(input_date, &description)
        .expect("Please use the format YYYY-MM-DD HH:MM:SS");

    // 4. Define our Metric interval (1 Gigasecond)
    let gigasecond = Duration::seconds(GIGASECOND);

    // 5. Calculate the future date
    let anniversary = start_date + gigasecond;

    println!("Starting point: {}", start_date);
    println!("Your 1 Gigasecond Anniversary is: {}", anniversary);
}