#[allow(dead_code)]
pub fn is_even(num: i32) -> bool {
    num % 2 == 0 // In Rust, the last expression is implicitly returned; no 'return' keyword needed
}

#[allow(dead_code)]
pub fn enter_number() -> bool {
    loop {
        let mut input: String = String::new();
        println!("Enter a number: ");

        let num = match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(n) => {
                        println!("You entered: {}", n);
                        n
                    }
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue;
                    }
                } // Fixed: Missing closing brace for the inner match
            } // Fixed: Missing closing brace for the Ok(_) arm
            Err(e) => {
                println!("Error reading input: {}", e);
                continue;
            }
        }; // Fixed: Added closing brace and semicolon for the outer assignment match

        return is_even(num);
    }
}
