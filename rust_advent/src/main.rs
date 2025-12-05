use std::io;
use rand::Rng;

fn main() {
    // secret is i32 now → no more comparison weirdness!
    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    // DEBUG LINE – comment it out or delete when you want real mystery
  //  println!("DEBUG: the secret number is {secret} (shhh, don't tell anyone)");

    let mut attempts = 0;
    let max_attempts = 10;

    println!();
    println!("Guess the number (1-100)! You got {max_attempts} tries!");
    println!("(type 'quit' or 'q' to give up like a city slicker)");
    println!("----------------------------------------------------");

    loop {
        attempts += 1;

        // ran out of tries?
        if attempts > max_attempts {
            println!("Game over, partner! The secret was {secret}.");
            println!("Better luck next time!");
            break;
        }

        println!("Attempt {attempts}/{max_attempts} → Enter your guess:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim();

        // quit command
        if input == "quit" || input == "q" {
            println!("You chickened out! The number was {secret}!");
            break;
        }

        // parse the guess
        let guess: i32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("that ain't a number, cowboy! try again (doesn't count as a guess)");
                attempts -= 1; // be nice
                continue;
            }
        };

        // range check
        if guess < 1 || guess > 100 {
            println!("i said 1 to 100! that guess doesn't count");
            attempts -= 1;
            continue;
        }

        // actual comparison
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Low!"),
            std::cmp::Ordering::Greater => println!("High!"),
            std::cmp::Ordering::Equal => {
                println!("YEEHAW! You guessed it in {attempts} attempt(s)!");
                if attempts == 1 {
                    println!("First-try legend!");
                } else if attempts <= 4 {
                    println!("Hot dang, you're sharp!");
                }
                break;
            }
        }
    }

    println!();
    println!("Thanks for playin', rust ranger!");
}