#[derive(Debug)] // basically means, that rust to convert our type to a form that can be used or is readable or as astring. 
struct Rectangle {
    width: u32,
    length: u32,
}

// methods
#[allow(dead_code)]
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn descriptive(&self) -> String {
        format!("Rectangle: {} x {}", self.width, self.length)
    }
}

#[allow(dead_code)]
pub fn calculate_area(width: u32, length: u32) -> u32 {
    let rectangle1: Rectangle = Rectangle { width, length };

    println!("Area of rectangle is: {}", rectangle1.area());
    println!("Descriptive: {}", rectangle1.descriptive());
    rectangle1.area()
}
