use std::io::{self, BufRead, Write};

// Disables compiler warning for the flush
#[allow(unused_must_use)]
/// # Parse
///
/// ### A personalised version of `io::stdin()`.
///
/// Create an instance of `io::stdin()` with the functionality of printing a piece of text before it.
/// The function will return the result of `io::stdin()` as a `String`
///
/// # Examples
/// ```
/// parse("Input letter > ");
/// // Terminal: Input letter > |
/// ```
///
/// ### returns String the user put in
///
pub fn parse(text: &str) -> String {
    // Since io::stdin() is a buffered function and the print!() macro is not we need to manually flush this to the output.
    // Without this the line will print after the stdin() is handled.
    print!("{}", text);
    io::stdout().flush();

    // We make a new mutable String where we will store the result of the stdin() in.
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    // Remove whitespace from String, turn it back into a String and return it.
    String::from(line.trim())
}

/// # Get indexes
///
/// Given string and a letter, find all the occurances of the letter inside the string.
///
/// # Examples
/// ```
/// let word = String::from("hello");
/// let letter = String::from("l");
/// get_indexes(&word, &letter);
/// // Output: [2, 3];
pub fn get_indexes(input: &String, letter: &String) -> Vec<usize> {
    // Get a vector of all the key value pairs of the letter and the index in the word.
    let interator_vec: Vec<_> = input.match_indices(letter).collect();

    // Create an empty vector where we will push the index numbers in
    let mut index_vec: Vec<usize> = Vec::new();

    // Loop over the length of the original vector and push all the indexes into the correct vector
    for i in 0..interator_vec.len() {
        index_vec.push(interator_vec[i].0);
    }

    // Return the vector with only the index numbers in it
    index_vec
}

/// # Enter in underscored
///
/// Enter letters into a vector composed of underscores.
///
/// ## Parameters
/// * word
///
/// This is the word the player is trying to guess in the game of hangman
/// ___
/// * guessed_letter
///
/// This is the letter the player is guessing to be in the word the player is trying to guess
/// ___
/// * underscored
///
/// This is the `Vector<String>` of underscores where we will insert the letter the player guessed into if the word contains it
/// ___
/// * indexes
///
/// This is the `Vector<usize> composed of the indexes of the letters inside the word the player is trying to get
/// We get this `Vector<usize>` from `get_indexes()`
///
/// # Examples
/// ```
/// let word = String::from("hello");
/// let letter = String::from("l");
/// let underscored_word = vec![String::from("_"); word.len()]; // ["_", "_", "_", "_", "_",];
/// let indexes = get_indexes(&word, &letter); // [2, 3];
///
/// enter_in_underscored(&word, &letter, underscored_word, &indexes)
/// // Returns: ["_", "_", "l", "l", "_"];
pub fn enter_in_underscored(
    word: &String,
    guessed_letter: &String,
    mut underscored: Vec<String>,
    indexes: &Vec<usize>,
) -> Vec<String> {
    // We check if the word contains the letter we put in
    if word.contains(guessed_letter) {
        // If the word contains the letter we loop over the indexes array
        for i in 0..indexes.len() {
            // For every entry in the indexes array we replace the underscore at that index in the underscored_word variable
            underscored[indexes[i]] = guessed_letter.clone();
        }
    }

    // We return the edited underscored word
    underscored.to_owned()
}
