use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let number_to_guess = rand::thread_rng().gen_range(0..101);
    let mut min: u32 = 1;
    let mut max: u32 = 100;
    let mut tries: u32 = 5;
    println!("Guess the number.");
    loop {
        let mut guess = String::new();
        println!("---------------------");
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&number_to_guess) {
            Ordering::Less => {
                if guess > min {
                    min = guess;
                }
            }
            Ordering::Greater => {
                if guess < max {
                    max = guess;
                }
            }
            Ordering::Equal => {
                println!("Correct.");
                println!("---------------------");
                break;
            }
        }
        tries -= 1;
        if tries == 0 {
            println!("Defeat, the number was: {}.", number_to_guess);
            println!("---------------------");
            break;
        }
        println!(
            "Value is between {} and {}.\nTries left: {} of 5.",
            min, max, tries
        );
    }
}
