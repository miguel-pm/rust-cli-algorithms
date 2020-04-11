use std::io;

mod sum;
mod largest;
use sum::sum_cli;
use largest::largest_cli;

pub fn cli () {
    let mut input = String::new();

    println!("\nWelcome to Miguelo's awesome calculator.
    Please select an operation you would like to perform:
    1 -> Sum
    2 -> Largest");

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
                2 => largest_cli(),
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
