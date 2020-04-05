use std::io;

fn run (n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => run(n - 1) + run(n - 2),
    }
}

pub fn cli () {
    let mut input = String::new();

    println!("\nWelcome to the Fibonacci number calculator.
    \nPlease select an index you would like to know:\n");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: Option<u32> = match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None
    };
    match n {
        Some(i) => {
            println!("Executing fibonacci sequence for {} numbers", i);
            let result = run(i);
            println!("Result: {}", result);
        },
        None => {
            println!("Please select a valid option! It has to be a positive integer no larger than 32 bits.");
            cli()
        }
    }
}
