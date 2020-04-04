mod functions;
mod algorithms;

use algorithms::fizz_buzz;
use functions::arithmetic_functions;

fn main() {
    let list = vec![1, 2, 3];
    let sum = arithmetic_functions::sum_list(&list[..]);
    println!("List sum: {:?}", sum);

    let avg = arithmetic_functions::avg(&list[..]);
    println!("Average of values: {:?}", avg);

    fizz_buzz::run();
}
