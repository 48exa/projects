mod hangman;
mod parser;
use crossterm_cursor::cursor;
use hangman::hangman_stage;
use parser::{enter_in_underscored, get_indexes, parse};
use press_btn_continue::wait;

fn main() {
    // We get the word that the other player will guess
    let word_to_guess = parse("Word to guess: ");

    // Turn the word into a vector containing an '_' for every letter in the word
    // By using the vec! macro we can make a vector containing a single character (_) times the length of the string we enter.
    let mut word_underscored = vec![String::from("_"); word_to_guess.len()];

    // Initialise an empty vector where we will store the picked letters
    let mut picked_letters: Vec<String> = vec![];

    // Initialise a counter of to increment the losses
    // We tell the compiler this is a u8 to overwrite the default allocated memory of an i32
    let mut losses: u8 = 0;

    // We make a vector of every letter of the word_to_guess
    // We do this to later compare this vector the the changed word_underscored vector
    // If they are the same the game ends
    let mut word_vec: Vec<String> = Vec::new();
    for _c in word_to_guess.chars() {
        word_vec.push(_c.to_string());
    }

    // Clear the console and se the cursor to the top of the screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // Start the gameloop
    loop {
        // Print the hangman based on the amount of losses you have
        hangman_stage(losses.into());

        // Print the underscored word vector without the apostrophes, commas and brackets
        // We do the same for the vector of chosen letters
        pretty_print("", &word_underscored);
        pretty_print("Chosen letters: ", &picked_letters);

        // Check if the word_underscored is equal to the vector of the word_to_guess
        // If this is correct we tell the player that they won and we break the loop, ending the game
        if word_underscored == word_vec {
            println!("You guessed the word!\nIt was {}", word_to_guess);
            wait("Press any key to close...").unwrap();
            break;
        }

        // If the losses are equal to 6 the player has ran out of guesses and loses the game
        // When this evaluates to true we tell the player they lost and break the loop, ending the game
        if losses == 9 {
            println!("You lost :(\n it was {}", word_to_guess);
            wait("Press any key to close...").unwrap();
            break;
        }

        // We get a new letter from the player that he wants to guess
        let guessed_letter = guess_letter();

        // Check if the letter chosen is already guessed, if so we prompt the player to choose a new letter
        // After getting a new letter we start the loop over again
        if picked_letters.contains(&guessed_letter) {
            println!("You already guessed {}!", &guessed_letter);
            pretty_print("Letters you guessed before: ", &picked_letters);
            continue;
        }

        // After picking a (correct) letter we add this letter into the vector of chosen letters
        picked_letters.push(guessed_letter.clone());

        // We enter the letter into the underscored word
        // See parser::enter_in_underscored for the explanation to this function
        word_underscored = enter_in_underscored(
            &word_to_guess,
            &guessed_letter,
            word_underscored,
            &get_indexes(&word_to_guess, &guessed_letter),
        );

        // If the guessed letter is not in the word_to_guess we increment the losses int
        if !word_to_guess.contains(&guessed_letter) {
            losses += 1;
        }

        // We move the cursor to the position of the word_underscore in the output
        cursor().goto(0, 8).expect("Failed to read");

        // We remove the underscored_word with \r and then write the new underscored_word over it
        pretty_print("\r", &word_underscored);

        // We remove the chosen letters the same way as above and overwrite the picked_letters in the output
        pretty_print("\rChosen letters: ", &picked_letters);

        // We move the cursor back to the top of the output and clear the output afterwards
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // The loop starts over
    }
}

/// # Guess Letter
///
/// We parse a string named `guessed_letter`
/// After parsing `guessed_letter` we start a `match` statement inside of a `loop`
///
/// If the `.len()` of `guessed_letter` is 1, we return the string
/// If the `.len()` of `guessed_letter` is 0 we tell the player to enter a letter and ask again
/// If the `.len()` of `guessed_letter` is anything else we prompt the player to enter a new letter
///
/// ### Returns -> String
fn guess_letter() -> String {
    let mut guessed_letter = parse("Letter to guess: ");

    loop {
        match guessed_letter.len() {
            1 => break guessed_letter,
            0 => {
                println!("Input a letter");
                guessed_letter = parse("Letter to guess: ");
                return guessed_letter;
            }
            _ => {
                println!("Input only 1 letter.");
                guessed_letter = parse("Letter to guess: ");
                return guessed_letter;
            }
        }
    }
}

/// # Pretty print
///
/// Loop over a vector and print every character inside of it and print it to get rid of the `[]`, `"` and `,`
/// You can choose to enter a string to print infront of the formatted vector as well.
///
/// # Examples
/// ```
/// let vector: Vec<String> = ["H", "e", "l", "l", "o"];
/// pretty_print("A beautiful formatted vector: ", &vector);
/// // Output: "A beautiful formatted vector: Hello"
/// ```
fn pretty_print(text: &str, vec: &Vec<String>) {
    print!("{}", text);
    for i in 0..vec.len() {
        print!("{}", vec[i])
    }
    println!();
}
