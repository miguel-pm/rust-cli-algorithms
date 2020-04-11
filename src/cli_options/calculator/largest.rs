use std::io;
use crate::arithmetic_functions;

pub fn largest_cli () {
    let mut numbers: Vec<i128> = Vec::new();
    loop {
        let mut input = String::new();
        println!("Please provide an entry for the list, once finnish just type 'done'.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "done" {
            println!("Alright, finnishing input");
            break;
        }

        let number: Option<i128> = match input.trim().parse() {
            Ok(n) => Some(n),
            Err(_) => None
        };
        if number == None {
            println!("That is not a valid number, starting over.");
            continue;
        }
        println!("Adding {} to the list", number.unwrap());
        numbers.push(number.unwrap());
    }
    let amount = numbers.len();
    println!("Processing list of {} numbers", amount);

    let largest_number = arithmetic_functions::largest(&numbers);
    if largest_number == None {
        println!("Your impervious list was empty genious");
        return;
    }
    println!("The largest number is: {}", largest_number.unwrap());
}
