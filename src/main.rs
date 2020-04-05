use std::io;

mod arithmetic_functions;
mod cli_options;

use cli_options::calculator;
use cli_options::fibonacci;
use cli_options::fizz_buzz;

fn main() {
    println!("Welcome to the awsome Miguelo's Rust CLI!");
    println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();

        println!("\nSelect one of  the following options:
        1 -> Calculator
        2 -> Fibonacci Number generator
        3 -> FizzBuzz Printer");

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        if n.trim() == "quit" {
            println!("Thank you for using Miguelo's software incorporated. Byeee!");
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please select a valid option!");
                continue
            },
        };
        println!("You chose {}", n);
        match n {
            1 => calculator::cli(),
            2 => fibonacci::cli(),
            3 => {
                println!("Launching Fizz Buzz Execution!");
                fizz_buzz::run();
                println!("There! I hope you liked that :)");
            },
            _ => {
                println!("Please select a valid option!");
                continue
            }
        }
    }
}
