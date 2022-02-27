use std::io::stdin;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

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

fn largest_u64(vector: &[u64]) -> u64 {
    let mut largest = vector[0];
    for &item in vector.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn is_u64_in(vector: &[u64], int: &u64) -> bool {
    let mut is_in = false;
    for &item in vector.iter() {
        if item == *int {
            is_in = true;
            break;
        }
    }
    is_in
}

fn main() {
    loop {
        println!("Welcome to PC's Fraction Simplifier. \nPlease enter the numerator:");

        let mut numerator_string = String::new();
        stdin().read_line(&mut numerator_string).expect("ERROR: Failed to read numerator from user.");

        let mut numerator: u64 = 0;
        let numerator_ref = &mut numerator;
        match numerator_string.trim().parse::<u64>() {
            Ok(v) => {*numerator_ref = v},
            Err(_) => {query_retry!("Invalid input. Your input was not a number, or exceeded the 64 bit uint limit (18,446,744,073,709,551,615).");}
        }
        if numerator < 1 {
            query_retry!("Invalid input. The numerator cannot be less than one.");
        }

        println!("Please enter the denominator:");

        let mut denominator_string = String::new();
        stdin().read_line(&mut denominator_string).expect("ERROR: Failed to read denominator from user.");

        let mut denominator: u64 = 0;
        let denominator_ref = &mut denominator;
        match denominator_string.trim().parse::<u64>() {
            Ok(v) => {*denominator_ref = v},
            Err(_) => {query_retry!("Invalid input. Your input was not a number, or exceeded the 64 bit uint limit (18,446,744,073,709,551,615).");}
        }
        if denominator < 1 {
            query_retry!("Invalid input. The denominator cannot be less than one.");
        }

        let mut numerator_factors:   Vec<u64> = [].to_vec();
        let mut denominator_factors: Vec<u64> = [].to_vec();
        let mut common_factors:      Vec<u64> = [].to_vec();

        let start_time = Instant::now();

        //let num_bar = ProgressBar::new(numerator);
        let num_bar = ProgressBar::new(numerator);
        num_bar.set_style(ProgressStyle::default_bar()
            .template("[{elapsed}] {pos:7.yellow}/{len:7.green} ({percent}%) {msg}"));
        num_bar.set_message("Calculating numerator factors...");
        for num in 1..=numerator {
            if numerator % num == 0 {
                numerator_factors.extend(&vec![num]);
            }
            num_bar.inc(1);
        }
        num_bar.finish();

        let den_bar = ProgressBar::new(denominator);
        den_bar.set_style(ProgressStyle::default_bar()
            .template("[{elapsed}] {pos:7.yellow}/{len:7.green} ({percent}%) {msg}"));
        den_bar.set_message("Calculating denominator factors...");
        for num in 1..=denominator {
            if denominator % num == 0 {
                denominator_factors.extend(&vec![num]);
            }
            den_bar.inc(1);
        }
        den_bar.finish();

        let com_bar = ProgressBar::new_spinner();
        com_bar.set_message("Calculating common factors...");
        if largest_u64(&numerator_factors) >= largest_u64(&denominator_factors) {
            for num in numerator_factors.iter() {
                if is_u64_in(&denominator_factors, num) {
                    common_factors.extend(&vec![*num]);
                }
            }
        } else {
            for num in denominator_factors.iter() {
                if is_u64_in(&numerator_factors, num) {
                    common_factors.extend(&vec![*num]);
                }
            }
        }
        com_bar.finish();

        let highest_common_factor = largest_u64(&common_factors);

        let final_answer: [u64; 2] = [numerator / highest_common_factor, denominator / highest_common_factor];

        let elapsed_time = start_time.elapsed().as_millis() as f64 / 1000f64;
        println!("\nCalculations are finished. \nThe simplified form of {}/{} is {}/{}. Time taken: {}", 
            numerator,
            denominator,
            final_answer[0],
            final_answer[1],
            elapsed_time
        );

        break;
    }
}
