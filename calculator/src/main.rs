use std::io;

fn calculate(num1: f32, num2: f32, sign: &str) {
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

fn choose_numbers() -> (f32, f32) {
    let mut num1 = String::new();
    let mut num2 = String::new();

    loop {
        println!("Choose your first number");
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");

        let mut _num1: f32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Make sure that you enter a *number*");
                continue;
            }
        };

        println!("Choose your second number");
        io::stdin()
            .read_line(&mut num2)
            .expect("failed to read line.");

        let _num2: f32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Make sure that you enter a *number*");
                continue;
            }
        };
        return (_num1, _num2);
    }
}

fn choose_sign() -> String {
    println!("Choose a sign: '+', '-', '*', '/' ");

    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        match buffer.trim() {
            "+" => break buffer,
            "-" => break buffer,
            "*" => break buffer,
            "/" => break buffer,
            _ => {
                println!("Invalid sign! Please choose from '+', '-', '*' or ':'");
                continue;
            }
        }
    }
}

fn main() {
    println!("Welcome to the calculator.");
    let binding = choose_sign();
    let sign = binding.trim();
    let nums = choose_numbers();

    calculate(nums.0, nums.1, sign);

    println!("Press [enter] to quit.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    todo!("
        Create a calculator that takes in a calculation (example: 3 + 3 - 29) and use
        something like split_whitespace to then iterate through the list with a [for x in spliced {{ sort by number or expression }}
        then run a calculation like how i do now. Later add control flow statements to make the calculator work with PEMDAS
    ");
}
