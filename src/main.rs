use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let numer_to_guess = rand::thread_rng().gen_range(1..101);
    println!("Guess the number.");
    loop {
        let mut guess = String::new();
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&numer_to_guess) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct.");
                break;
            }
        }
    }
}
