use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! from 1 to 20");

    let mut numbers_random = rand::thread_rng();
    let secret_number = numbers_random.gen_range(1..=20);

    loop {
        //String::new() = sebuah string baru dibuat, dan lifetime-nya dimulai. 
        let mut guess = String::new();
      

        // take whatever the user types into standard input and append that into a string (without overwriting its contents),
        io::stdin()
            // .read_line(&mut guess)
            .read_line(&mut guess)
            .expect("failed to read line");

        // shadowing
        // .expect("Please type a number!") ->> Rsult Error Handling (match OK / Err)

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if guess < 1 || guess > 20 {
            println!("The secret number will be between 1 and 20.");
            continue;
        }

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is: {}", secret_number);
                break;
            }
        }
    }
}

