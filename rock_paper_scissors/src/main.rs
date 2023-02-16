use std::io;
use std::io::stdin;
use std::sync::Mutex;

use rand::Rng;

static MOVES: [&str; 3] = ["rock", "paper", "scissors"];
static WINS: Mutex<u32> = Mutex::new(0);
static LOSSES: Mutex<u32> = Mutex::new(0);
static GAMES: Mutex<u32> = Mutex::new(0);

/// Compares the moves against each other and increments the wins, losses and games
fn compare() -> String {
    let usermove: String = get_input();
    let pcmove: String = generate().to_owned();

    //-1 for a loss, 0 for a tie, 1 for a win
    let mut _win: i8 = 0;

    // all winning cases
    if usermove == "rock" && pcmove == "scissors"
        || usermove == "paper" && pcmove == "rock"
        || usermove == "scissors" && pcmove == "paper"
    {
        _win = 1;
    } else if pcmove == "rock" && usermove == "scissors"
        || pcmove == "paper" && usermove == "rock"
        || pcmove == "scissors" && usermove == "paper"
    {
        _win = -1;
    } else {
        _win = 0;
    }

    match _win {
        1 => {
            *WINS.lock().unwrap() += 1;
        }
        -1 => {
            *LOSSES.lock().unwrap() += 1;
        }
        0 => {
            println!("You tied!");
        }
        _ => println!(
            "I am required to check for all, but my value can't be anything other than these"
        ),
    }

    *GAMES.lock().unwrap() += 1;

    pcmove
}

/// Generates a move for the PC
fn generate() -> &'static str {
    let r#move: &str = MOVES[rand::thread_rng().gen_range(0..=2)];
    r#move
}

/// Gets the user input and returns the String `input` containing it
fn get_input() -> String {
    loop {
        let mut input: String = String::new();

        stdin().read_line(&mut input).expect("Failed to read line.");

        if !MOVES.contains(&input.trim()) {
            println!("invalid move. Choose rock, paper or scissors.");
        } else {
            input = input.trim().to_owned();
            input = input.to_string();
            break input;
        }
    }
}

/// Gets the user defined max score once and never again
fn get_max_score() -> String {
    println!("To what score do you wish to play?");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn main() {
    println!("Let's play Rock Paper Scissors :)");
    let mut ask_max: bool = true;
    let mut max_score: u32 = 0;
    loop {
        if ask_max {
            println!("Choose a max score!");
            max_score = match get_max_score().trim().parse() {
                Ok(num) => {
                    ask_max = false;
                    num
                }
                Err(_) => {
                    println!("Invalid guess! Make sure you enter a number");
                    continue;
                }
            };
        }

        println!("Choose! (r)ock, (p)aper, (s)cissors");
        let pcmove: String = compare();

        match pcmove.as_str() {
            "rock" => println!("I chose rock"),
            "paper" => println!("I chose paper"),
            "scissors" => println!("I chose scissors"),
            _ => println!(
                "I am required to check for all, but my value can't be anything other than these"
            ),
        }

        println!(
            "wins: {}. losses: {}. games: {}\n",
            WINS.lock().unwrap(),
            LOSSES.lock().unwrap(),
            GAMES.lock().unwrap()
        );

        if *WINS.lock().unwrap() > max_score - 1 {
            println!("You won!");
            break;
        } else if *LOSSES.lock().unwrap() > max_score - 1 {
            println!("You lost!");
            break;
        }
    }
    println!("Thanks for playing! Hope you enjoyed\nPress [ENTER] to quit!");
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}
