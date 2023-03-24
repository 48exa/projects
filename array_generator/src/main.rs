use colored::Colorize;
use std::io;

fn print_with_apostrophes(string: &str) {
    let char_vec: Vec<char> = string.chars().collect();

    print!("\nYour input line, formatted as an array: \nlet array = [");

    for c in char_vec {
        if String::from(c) == "\"".to_string() || String::from(c) == "\\".to_string() {
            print!("\"\\{}\", ", String::from(c))
        } else {
            print!("\"{}\", ", String::from(c));
        };
    }

    print!("];\n");
}

fn main() {
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            println!("{}\n{}",
        "String splitter into array by 48exa".white().on_green(), 
        "Write your string below, spaces are allowed.\nIf you whatever reason need a space in your array...".yellow().on_black());
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            print_with_apostrophes(input.trim());

            println!("\n{}", "Press [enter] to quit.".black().on_white());
            io::stdin().read_line(&mut String::new()).expect(" ");
        }
        Err(_) => {}
    }
}
