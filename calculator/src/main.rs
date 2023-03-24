use std::io;

fn input() -> String {
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    buf.to_string()
}

fn conv_f64(mut string: String) -> f64 {
    loop {
        let converted: f64 = match string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input string was not a number, try again please");
                string = input();
                continue;
            }
        };
        return converted
    }
}

fn choose_sign() -> String {
    println!("Choose a sign: '+', '-', '*', '/' ");

    let mut binding = input();
    let mut sign = binding.trim();

    loop {
        match sign {
            "+" | "-" | "*" | "/" => break String::from(sign),
            _ => {
                println!("Invalid sign! Please choose from '+', '-', '*' or '/'");
                binding = input();
                sign = binding.trim();
                continue;
            }
        }
    }
}

fn choose_numbers() -> (f64, f64) {
    loop {
        println!("Choose your first number");
        let num1 = conv_f64(input());

        println!("Choose your second number");
        let num2 = conv_f64(input());

        return (num1, num2);
    }
}

fn calculate(num1: f64, num2: f64, sign: &str) {
    match sign {
        "+" => println!("{} + {} = {}", num1, num2, num1 + num2),
        "-" => println!("{} - {} = {}", num1, num2, num1 - num2),
        "*" => println!("{} * {} = {}", num1, num2, num1 * num2),
        "/" => println!("{} : {} = {}", num1, num2, num1 / num2),
        _ => {
            println!("Error");
        }
    }
}

fn main() {
    println!("Welcome to the calculator.");
    let sign = String::from(choose_sign().trim());
    let nums = choose_numbers();

    calculate(nums.0, nums.1, &sign);

    println!("Press [enter] to quit.");
    input();
}
