use std::io;

use crate::arithmetic_functions;

fn sum_cli () {
    let mut input1 = String::new();

    println!("\nPerforming a sumation. Please input the first number to add:");

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    let first_num: Option<i128> = match input1.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None
    };
    if first_num == None {
        println!("That is not a valid number, starting over.");
        sum_cli()
    }
    println!("\nRight. Now input the second number to add:");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let second_num: Option<i128> = match input2.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None
    };
    if second_num == None {
        println!("That is not a valid number, starting over.");
        sum_cli()
    }
    let result = arithmetic_functions::sum_list(&[first_num.unwrap(), second_num.unwrap()]);
    println!("Your result is: {}", result);
}

pub fn cli () {
    let mut input = String::new();

    println!("\nWelcome to Miguelo's awesome calculator.
    Please select an operation you would like to perform:
    1 -> Sum");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: Option<u32> = match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None
    };
    match index {
        Some(i) => {
            match i {
                1 => sum_cli(),
                _ => {
                    println!("You chose {} and that option is not contemplated yet, please select a valid one", i);
                    return cli()
                }
            }
        },
        None => {
            println!("Please select a valid option! It has to be a positive integer no larger than 32 bits.");
            cli()
        }
    }
}
