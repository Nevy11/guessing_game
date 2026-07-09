#[allow(dead_code)]
pub fn addition(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}

#[allow(dead_code)]
pub fn subtraction(a: i32, b: i32) -> i32 {
    let c = a - b;
    return c;
}

#[allow(dead_code)]
pub fn multiplication(a: i32, b: i32) -> i32 {
    let c = a * b;
    return c;
}

#[allow(dead_code)]
pub fn division(a: i32, b: i32) -> i32 {
    let c = a / b;
    return c;
}

#[allow(dead_code)]
pub fn modulus(a: i32, b: i32) -> i32 {
    let c = a % b;
    return c;
}

#[allow(dead_code)]
pub fn run() {
    let a = rand::random_range(1..10);
    let b = rand::random_range(1..10);
    println!("Random numbers: a = {}, b = {}", a, b);
    println!("Addition: {} + {} = {}", a, b, addition(a, b));
    println!("Subtraction: {} - {} = {}", a, b, subtraction(a, b));
    println!("Multiplication: {} * {} = {}", a, b, multiplication(a, b));
    println!("Division: {} / {} = {}", a, b, division(a, b));
    println!("Modulus: {} % {} = {}", a, b, modulus(a, b));
}

#[allow(dead_code)]
pub fn loop_run() {
    let mut count = 0;
    loop {
        run();
        count += 1;
        if count == 10 {
            break;
        }
    }
}
