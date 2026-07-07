use std::io;

#[allow(dead_code)]
pub fn say_hello() {
    println!("Enter your name: ");
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name.trim_end()),
        Err(err) => println!("Error: {}", err),
    }
}

#[allow(dead_code)]
pub fn add_numbers() {
    loop {
        let a: i32 = rand::random_range(1..100);
        let mut b: String = String::new();
        println!("Enter a number to add to {}: ", a);
        match io::stdin().read_line(&mut b) {
            Ok(_) => {
                let b: i32 = match b.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue;
                    }
                };
                let sum: i32 = a + b;
                println!("The sum of {} and {} is {}", a, b, sum);
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
