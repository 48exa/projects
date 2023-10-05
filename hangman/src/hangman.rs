/// # Hangman 'Pics'
///
/// Get a certain stage of the hangman
///
/// # Examples
///
/// ```
/// hangman_stage(0);
/// /* Output:
///   +---+
///   |   |
///       |
///       |
///       |
///       |
/// =========
/// */
///
/// hangman_stage(4);
/// /* Output
///   +---+
///   |   |
///   O   |
///  /|\  |
///       |
///       |
/// =========
/// */
///
/// hangman_stage(99);
/// // Output: Index out of bounds: Index 99 while array is 7
/// ```
pub fn hangman_stage(index: usize) {
    let hangman = [
        "
========",
        "
      |
      |
      |
      |
      |
========",
        "
  +---+
      |
      |
      |
      |
      |
=========",
        "
  +---+
  |   |
      |
      |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
      |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
=========",
    ];

    if index > hangman.len() {
        println!(
            "Index out of bounds: Index {} while array is {}",
            index,
            hangman.len()
        );
    }

    println!("{}", hangman[index])
}
