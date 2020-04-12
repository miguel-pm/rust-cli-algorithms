use std::io;

mod longest;
use longest::longest_cli;

pub fn cli () {
    let mut input = String::new();

    println!("\nWelcome to Miguelo's string manipulation and analisis CLI.
    Please select an operation you would like to perform:
    1 -> Longest");

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
                1 => longest_cli(),
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
