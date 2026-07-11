#[allow(dead_code)]
pub fn variable_scope_example() {
    println!("Variable Scope Example:");
    {
        let x: i32 = 5;
        println!("The value of x is: {}", x);
    }
    // println!("The value of x is: {}", x); // This will cause a compile-time error because x is out of scope
}

#[allow(dead_code)]
pub fn string_manipulation() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
}

#[allow(dead_code)]
pub fn sharing_data() {
    // Start with a number then a string
    let x: i32 = 10;
    let y: i32 = x;
    println!("x: {}, y: {}", x, y); // Both x and y can be used because i32 implements the Copy trait
    let s1: String = String::from("Hello");
    let s2: String = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("s1: {}", s1); // This will cause a compile-time error because s1 has been moved
    println!("s2: {}", s2);

    {
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // s1 is cloned to s2, both are valid
        println!("s1: {}, s2: {}", s1, s2);
        // This works, .clone() method, since it's a deep copy, both s1 and s2 are valid and can be used independently
    }
}
