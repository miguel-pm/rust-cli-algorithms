use std::io;
use crate::string_functions;

pub fn longest_cli () {
    let mut strings: Vec<String> = Vec::new();

    println!("Please provide two words, the program will resolve which is the longest.");
    for i in [1, 2].iter() {
        let mut input = String::new();
        println!("Word at index {}: ", i);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        strings.push(String::from(input.trim()));
    }
    let longest_word = string_functions::longest(&strings[0], &strings[1]);
    println!("The longest input word is {}", longest_word);
}
