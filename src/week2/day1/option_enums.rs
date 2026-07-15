#[allow(dead_code)]
pub fn use_option_enum() {
    let any_number: Option<f64> = Some(0.5);
    let any_name: Option<String> = Some(String::from("Coder".to_string()));
    let is_class: Option<i32> = None;
    println!(
        "any_number: {:?}, any_name: {:?}, is_class: {:?}",
        any_number, any_name, is_class
    );
}
