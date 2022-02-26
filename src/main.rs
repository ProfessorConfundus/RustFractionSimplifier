use std::io::stdin;

macro_rules! query_retry {
    ($error_msg:tt) => {
        println!("{} \nWould you like to run the program again? (y/N) ", $error_msg);
        
        let mut input = String::new();
        stdin().read_line(&mut input)
            .ok()
            .expect("ERROR: Failed to read input from user.");
        
        if input == "" || input.to_lowercase().trim() == "n" {
            println!("Exiting...");
            break;
        } else if input.to_lowercase().trim() == "y" {
            println!("Re-running...\n--------------------");
            continue;
        } else {
            println!("Invalid input. Exiting...");
            break;
        }
    };
}

fn main() {
    loop {
        println!("Welcome to PC's Fraction Simplifier. \nPlease enter the numerator:");

        let mut numerator_string = String::new();
        stdin().read_line(&mut numerator_string)
            .ok()
            .expect("ERROR: Failed to read numerator from user.");

        let mut numerator: u64 = 0;
        let numerator_ref = &mut numerator;
        match numerator_string.trim().parse::<u64>() {
            Ok(v) => {*numerator_ref = v},
            Err(_) => {query_retry!("Invalid input. Your input was not a number");}
        }
        println!("{}", numerator)
    }
}
