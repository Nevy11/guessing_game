#[allow(dead_code)]
pub fn run() {
    test_mutability();
}

#[allow(dead_code)]
fn test_mutability() {
    let x = "Stephen";
    println!("{}", x);
    // x = "Samuel" // This will throw an error because x is immutable
    // default programming means that variables are immutable unless specified otherwise
    let x = "Samuel"; // This is called shadowing, we can reassign a variable with the same name
    println!("{}", x);
    let mut x = "Stephen"; // This is a mutable variable, we can reassign it
    println!("{}", x);
    x = "John"; // This is allowed because x is mutable.
    println!("{}", x);
}
