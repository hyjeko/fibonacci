use std::io;

fn main() {
    //run forever
    loop {
        //instructions
        println!("---------------------------------------------------");
        println!("Please Enter a Fibonacci number (u8: 0-255) to calculate");
        println!("---------------------------------------------------");
        //new mutable string to hold input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Not a valid u8 integer, {} \n", error);
                continue;
            }
        };
        //calculate and print result using all three methods
        let result_one = fibonacci(input.into());
        let result_two = tuple_fibonacci(input.into());
        let result_three = struct_fibonacci(input.into());
        assert_eq!(result_one, result_two); // if (a == b && a == c) then b == c
        assert_eq!(result_one, result_three);
        println!("Your Fibonacci number is: {}", result_three);
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
fn fibonacci(n: u128) -> u128 {
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

fn tuple_fibonacci(n: u128) -> u128 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut operands: (u128, u128) = (0, 1);
    let mut result = 1;

    for _i in 1..n {
        let prev_operands: (u128, u128) = (operands.0, operands.1);
        result = prev_operands.0 + prev_operands.1;
        operands = (prev_operands.1, result);
    }

    return result;
}

fn struct_fibonacci(n: u128) -> u128 {
    if n == 0 || n == 1 {
        return n;
    }
    struct Operands {
        first_number: u128,
        second_number: u128,
    }
    let mut operands = Operands {
        first_number: 0,
        second_number: 1,
    };
    let mut result = 1;

    for _i in 1..n {
        let prev_operands = Operands {
            first_number: operands.first_number,
            second_number: operands.second_number,
        };
        result = prev_operands.first_number + prev_operands.second_number;
        operands = Operands {
            first_number: prev_operands.second_number,
            second_number: result,
        }
    }

    return result;
}
