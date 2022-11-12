use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! from 1 to 20");

    let secret_number = rand::thread_rng().gen_range(1..=20);

    loop {
        let mut guess = String::new();

        // take whatever the user types into standard input and append that into a string (without overwriting its contents),
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // shadowing
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is: {secret_number}");
                break;
            }
        }
    }
}
