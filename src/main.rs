use std::io;

fn main() {
    //run forever
    loop {
        //instructions
        println!("---------------------------------------------------");
        println!("Please Enter a Fibonacci number (u64) to calculate");
        println!("---------------------------------------------------");
        //new mutable string to hold input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let valid_input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Not a valid u64 integer, {} \n", error);
                continue;
            }
        };
        //calculate and print result
        let result = fibonacci(valid_input);
        println!("Your Fibonacci number is: {}", result);
    }
}

/**
 * In mathematics, the Fibonacci numbers, commonly denoted Fn,
 * form a sequence, the Fibonacci sequence, in which each number
 * is the sum of the two preceding ones. The sequence commonly starts
 * from 0 and 1
 *
 * F0 = 0
 * F1 = 1
 * F2 = 0 + 1 = 1
 * F3 = 1 + 1 = 2
 * F4 = 1 + 2 = 3
 * F5 = 2 + 3 = 5
 * ...
 */
/**
 * @return Fn
 */
fn fibonacci(n: u64) -> u64 {
    // first two cases are pre-determined
    if n == 0 || n == 1 {
        return n;
    };
    let mut first_number = 0;
    let mut second_number = 1;
    let mut result = 1;

    for _i in 1..n {
        // save old vals
        let prev_first_number = first_number;
        let prev_second_number = second_number;
        // mutate new vals
        result = prev_first_number + prev_second_number;
        first_number = prev_second_number;
        second_number = result;
    }

    return result;
}
