use std::{fmt, io, str};

use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
enum MovesError {
    Unknown(String),
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

impl fmt::Display for MovesResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovesResult::Win(result) => match result {
                CompMoves::RockOverScissors => write!(f, "Rock beats scissors"),
                CompMoves::PaperOverRock => write!(f, "Paper beats rock"),
                CompMoves::ScissorsOverPaper => write!(f, "Scissors beats paper"),
            },
            MovesResult::Lose(result) => match result {
                CompMoves::RockOverScissors => write!(f, "Rock beats scissors"),
                CompMoves::PaperOverRock => write!(f, "Paper beats rock"),
                CompMoves::ScissorsOverPaper => write!(f, "Scissors beats paper"),
            },
            MovesResult::Tie => write!(f, "You tied!"),
        }
    }
}

impl Compare<Moves, MovesResult> for Moves {
    fn compare(&self, b: &Moves) -> MovesResult {
        match self {
            Moves::Rock => match b {
                Moves::Rock => MovesResult::Tie,
                Moves::Paper => MovesResult::Lose(CompMoves::PaperOverRock),
                Moves::Scissors => MovesResult::Win(CompMoves::RockOverScissors),
            },
            Moves::Paper => match b {
                Moves::Rock => MovesResult::Win(CompMoves::PaperOverRock),
                Moves::Paper => MovesResult::Tie,
                Moves::Scissors => MovesResult::Lose(CompMoves::ScissorsOverPaper),
            },
            Moves::Scissors => match b {
                Moves::Rock => MovesResult::Lose(CompMoves::RockOverScissors),
                Moves::Paper => MovesResult::Win(CompMoves::ScissorsOverPaper),
                Moves::Scissors => MovesResult::Tie,
            },
        }
    }
}

impl str::FromStr for Moves {
    type Err = MovesError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "rock" => Ok(Moves::Rock),
            "p" | "paper" => Ok(Moves::Paper),
            "s" | "scissors" => Ok(Moves::Scissors),
            _ => Err(MovesError::Unknown(s.to_string())),
        }
    }
}

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Moves::Rock => write!(f, "Rock"),
            Moves::Paper => write!(f, "Paper"),
            Moves::Scissors => write!(f, "Scissors"),
        }
    }
}

impl Distribution<Moves> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Moves {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => Moves::Rock,
            1 => Moves::Paper,
            2 => Moves::Scissors,
            _ => unreachable!(),
        }
    }
}

enum Moves {
    Rock,
    Paper,
    Scissors,
}

enum CompMoves {
    RockOverScissors,
    PaperOverRock,
    ScissorsOverPaper,
}

enum MovesResult {
    Win(CompMoves),
    Lose(CompMoves),
    Tie,
}

pub fn main() {
    loop {
        println!("Choose! (r)ock, (p)aper or (s)cissors!");

        let mut um: String = String::new();
        let cm: Moves = rand::thread_rng().gen();

        io::stdin().read_line(&mut um).expect("Failed to read line");

        let um: Moves = um.trim().parse().expect("This is not a valid guess");
        let result: MovesResult = um.compare(&cm);

        println!("You guessed {}", um);
        println!("I gussed {}", cm);
        println!("{}", result);
    }
}