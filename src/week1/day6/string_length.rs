#[allow(dead_code)]
pub fn string_length_example(s: String) -> (String, usize) {
    let length = s.len();
    println!("The length of '{}' is {}.", s, length);
    (s, length)
}
