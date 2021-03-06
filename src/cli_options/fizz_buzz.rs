fn get_str_to_print (number: i32) -> String {
    if number % 3 == 0 {
        let mut res = String::from("Fizz");
        if number % 5 == 0 {
            res.push_str(" Buzz");
        }
        res.push_str("!");
        return res;
    }
    if number % 5 == 0 {
        return String::from("Buzz!");
    }
    number.to_string()
}

pub fn run () {
    for l in 0..101 {
        let res = get_str_to_print(l);
        println!("{}", res);
    }
}
