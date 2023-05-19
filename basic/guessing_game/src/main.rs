use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;
    
    loop {
        println!("Input the number: ");
        guesses += 1;
        // mut means mutable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        //println!("Secret Number: {secret_number}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("You guessed it right");
                println!("It took you {guesses} guesses");
                break;
            }
        }
    }
   
}
