use crate::week1::day6;

#[allow(dead_code)]
pub fn return_ownership() -> String {
    let s = String::from("Hello");
    s // Ownership of s is returned to the caller
}

pub fn take_and_giveback_ownership(s: String) -> String {
    s // Ownership of s is returned to the caller
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run_main() {
    let s = return_ownership(); // just returns ownership of the string to s
    println!("Returned ownership: {}", s);
    let s1 = String::from("Hello");
    let s2 = take_and_giveback_ownership(s1); // s1 is moved to s2, s1 is no longer valid
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This will cause a compile-time error because s1 has been moved
    let (s3, length) = day6::string_length::string_length_example(String::from("Hello, world!"));
    println!("s3: {}, length: {}", s3, length);
}
