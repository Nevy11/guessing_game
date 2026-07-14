mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
fn main() {
    day7::structures::record_user(
        String::from("John"),
        String::from("Doe"),
        30,
        String::from("Male"),
        true,
    );

    let area = day7::methods::calculate_area(5, 10);
    println!("{area}, is just like above");
}
