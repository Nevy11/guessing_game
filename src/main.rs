use std::{cmp::Ordering, io};

#[allow(dead_code)]
const MAX_NUMBER: i32 = 100;
fn main() {
    // say_hello();
    // take_input_from_user();
    // guess_random_number();
    // comparing_num();
    handling_invalid_input();
}

// Saying hello to the user
fn _say_hello() {
    println!("Hello, world");
}

// Taking input from the user
fn _take_input_from_user() {
    println!("Hey, type something here: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("type something!!!");
    println!("You've typed: {:?}", input);
}

// Guessing a random number
fn _guess_random_number() {
    println!("Guess a number between 1 and {}: ", MAX_NUMBER);
    let random_number: i32 = rand::random_range(1..=MAX_NUMBER); // this is a range, and the range is inclusive, so it will generate a random number between 1 and 100, inclusive
    // i think it's an update
    // which we understand
    // as "generate a random number between 1 and 100, inclusive" (1 and 100 are included in the range)
    // but if i say 1..100, it will generate a random number between 1 and 99, inclusive (1 is included but 100 is not)
    println!("Random number is: {}", random_number);
    println!("Type your guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read a line");
    println!("Your guess is: {}", guess);
}
// This course is for learning, and here there is no learning, just pressing tab and everything is fill

// trying comparing lesser number to greater number
fn _comparing_num() {
    // the loop statement, loops it until there is a break statement, or the program is terminated
    loop {
        let secret_number = rand::random_range(1..=100);
        println!("Secret number is: {}", secret_number);

        println!("Type a number: ");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read the line");

        let num1 = num1.trim().parse::<i32>().expect("Please type a number!");
        match num1.cmp(&secret_number) {
            Ordering::Less => println!("Your number is less than the secret number"),
            Ordering::Greater => println!("Your number is greater than the secret number"),
            Ordering::Equal => {
                println!("Your number is equal to the secret number, you won the game");
                break; // this statement, breaks the loop
            }
        }
    }
}

fn handling_invalid_input() {
    loop {
        let secret_number = rand::random_range(1..=100);
        println!("Secret number is: {}", secret_number);

        println!("Type a number: ");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read the line");

        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please type a valid number!\nError: {}", err);
                continue; // this statement, continues the loop
            }
        };

        match num1.cmp(&secret_number) {
            Ordering::Less => println!("Your number is less than the secret number"),
            Ordering::Greater => println!("Your number is greater than the secret number"),
            Ordering::Equal => {
                println!("Your number is equal to the secret number, you won the game");
                break; // this statement, breaks the loop
            }
        }
    }
}
