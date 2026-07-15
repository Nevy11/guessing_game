#[allow(dead_code)]
pub enum Color {
    Black,
    White,
    Brown,
}

#[allow(dead_code)]
pub fn return_color(color: Color) {
    match color {
        Color::Black => println!("Black"),
        Color::White => println!("White"),
        Color::Brown => println!("Brown"),
    }

    let name = Some(5);
    match name {
        Some(i32) => println!("number is i32"),
        None => println!("none variable"),
    }
}
