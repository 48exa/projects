use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess a number!");

    //generate random number using standard library gen_range()
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut _guesses: u32 = 0;


    loop {
        _guesses += 1;
        println!("Please input your guess.");

        //mutatable string guess that is equal to a new empty string
        let mut guess = String::new();

        //standard input that adds console input to mutatable string guess.
        //error handling to make the compiler happy
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //convert String type to unsigned 32 bit integer by trimming and parsing
        //the String guess. Error handling incase the original input to the String guess
        //is not a number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess! Make sure you enter a number");
                continue; 
            }
        };

        //match statement using standard output cmd::Ordering.
        //arms of the match statement evaluate if the input is smaller, bigger,
        //or equal to the u32 secret number. Arms for Less and Greater are
        //before the Equal arm to increase efficiency, as for most cases the
        //guess won't be equal to the secret number. So it is of least importance to check.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too big!"),
            Ordering::Equal => {
                println!("You guessed the correct number in {} guesses", _guesses);

                //opening an stdin() to make sure the .exe does not automatically close after
                //the loop exits. Error handling for the compiler.
                println!("You guessed correct! \n Press enter to quit!");
                io::stdin()
                    .read_line(&mut String::new())
                    .expect("Failed to read line");
                break;
            }
        }
    }
}