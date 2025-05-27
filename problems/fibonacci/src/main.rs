use std::io;

fn main() {
    println!("Hey buddy, Please enter input to find the fibonacci series!");
    let mut input_from_user: String = String::new();

    io::stdin().read_line(&mut input_from_user).expect("Please give valid input");

    let input_from_user : u32 = input_from_user.trim().parse().expect("Please enter valid positive number buddy!");


    for n in 0..input_from_user {
        println!("Fibonacci of {n} is {}",find_fibonacci(n));
    }

}

fn find_fibonacci(number: u32) -> u128 {
    if number == 1 || number == 2 {
        return 1;
    }
    let mut result: u128 = 0;
    let mut prev_one: u128 = 1;
    let mut prev_two:u128 = 1;
    for _n in 2..number {
        result = prev_one + prev_two;
        prev_two = prev_one;
        prev_one = result;
    }
    return result;
}
