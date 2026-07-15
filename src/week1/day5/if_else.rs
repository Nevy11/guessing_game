use std::cmp::Ordering;

// This function demonstrates the use of if-else statements in Rust.
#[allow(dead_code)]
pub fn if_else_example() {
    let x: i32 = 5;
    let y: i32 = 10;

    if x < y {
        println!("x is less than y");
    } else if x > y {
        println!("x is greater than y");
    } else {
        println!("x is equal to y");
    }
}

// This function demonstrates the use of match statements in Rust.
#[allow(dead_code)]
pub fn match_statements() {
    let x: i32 = 5;
    let y: i32 = 10;

    match x.cmp(&y) {
        Ordering::Less => println!("x is less than y"),
        Ordering::Greater => println!("x is greater than y"),
        Ordering::Equal => println!("x is equal to y"),
    }
}
