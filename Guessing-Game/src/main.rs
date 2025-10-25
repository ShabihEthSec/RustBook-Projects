use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("The Guessing Game!!!!");

    let secret_number = rand::rng().random_range(1..=99);
    
    println!("\nGuess the Number");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Unable to Parse");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too High!"),
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

    }

    
}
